use clap::Parser;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::{Protocol, SpanExporter, WithExportConfig};
use opentelemetry_sdk::trace::SdkTracerProvider;
use std::net::{Ipv4Addr, SocketAddr};
use tower_http::trace::TraceLayer;
use tracing::{instrument, Instrument, Level};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use axum::{
    extract::State,
    http::{HeaderName, HeaderValue},
    routing::get,
};

mod models;
use futures::future::join_all;
use ipnetwork::Ipv4Network;
use models::*;
use tokio::time::sleep;

// Helper function to iterate over IP addresses between start and end
fn ip_range(start: Ipv4Addr, end: Ipv4Addr) -> impl Iterator<Item = Ipv4Addr> {
    let start_u32 = u32::from(start);
    let end_u32 = u32::from(end);

    (start_u32..=end_u32).map(|ip_u32| Ipv4Addr::from(ip_u32))
}

#[instrument]
async fn spawn_scanner(
    cidr: &str,
) -> Result<tokio::sync::watch::Receiver<Vec<SocketAddr>>, reqwest::Error> {
    tracing::info!("🔍 Starting background inverter discovery");

    // Parse the network
    let network: Ipv4Network = cidr.parse().expect("Invalid CIDR format");

    // Get the starting IP address of the network and the number of addresses in the subnet
    let start_ip = network.network().clone();
    let end_ip = network.broadcast().clone();

    let (send, recv) = tokio::sync::watch::channel(vec![]);
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;

    tokio::task::spawn(async move {
        loop {
            async {
                tracing::info!("Starting network scan for inverters");
                let ips: Vec<_> = ip_range(start_ip, end_ip)
                    .filter(|ip| *ip != network.network() && *ip != network.broadcast())
                    .collect();
                let total_ips = ips.len();
                tracing::info!("Scanning {} IP addresses", total_ips);

                // Iterate over all the IP addresses in the subnet
                let results: Vec<SocketAddr> = join_all(ips.into_iter().map(|ip_addr| {
                    let client = client.clone();
                    async move {
                        let resp = client
                            .get(format!(
                                "http://{ip_addr}/components/BatteryManagementSystem/readable",
                            ))
                            .send()
                            .await;
                        let resp = match resp {
                            Ok(resp) => resp,
                            Err(err) => {
                                tracing::info!(?err, "Rejecting, bad response");
                                return None;
                            }
                        };
                        if resp.status() != 200 {
                            tracing::info!(status=?resp.status(), "Rejecting, bad status");
                            return None;
                        }
                        if resp.headers().get(HeaderName::from_static("content-type"))
                            != Some(&HeaderValue::from_static("text/javascript"))
                        {
                            tracing::info!("Rejecting, bad content type");
                            return None;
                        }
                        resp.remote_addr()
                    }
                    .instrument(tracing::info_span!("Scanning address", ?ip_addr))
                }))
                .await
                .into_iter()
                .filter_map(|r| r)
                .collect();
                tracing::info!(
                    "Scan complete. Found {} inverters: {:?}",
                    results.len(),
                    &results
                );
                send.send(results).unwrap();
            }
            .instrument(tracing::info_span!("networking_scan"))
            .await;
            tracing::info!("Waiting 5 minutes before next scan");
            sleep(std::time::Duration::from_secs(60 * 5)).await;
        }
    });

    Ok(recv)
}

#[derive(clap::Parser)]
struct Args {
    #[arg(default_value = "192.168.1.0/24")]
    default_network: String,
}

#[tokio::main]
async fn main() {
    // Get args
    let args = Args::parse();

    // Setup tracing
    let registry = tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env());
    if let Ok(otlp_exporter) = SpanExporter::builder()
        .with_http()
        .with_protocol(Protocol::HttpBinary)
        .build()
    {
        let sdk = SdkTracerProvider::builder()
            .with_batch_exporter(otlp_exporter)
            .build();
        let tracer = sdk.tracer("fronius-exporter");
        registry.with(OpenTelemetryLayer::new(tracer)).init();
    } else {
        registry.init();
    }

    // Span scanner
    let addrs = spawn_scanner(&args.default_network).await.unwrap();

    // Start webserver
    let app = axum::Router::new()
        .route("/health", get(health))
        .merge(
            axum::Router::new()
                .route("/metrics", get(metrics))
                .with_state(addrs),
        )
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[instrument]
async fn scrape(ip: SocketAddr) -> Metrics {
    let (req, req1, req2) = futures::join!(
        reqwest::get(format!(
            "http://{ip}/components/BatteryManagementSystem/readable"
        ))
        .await
        .unwrap()
        .json::<serde_json::Value>(),
        reqwest::get(format!("http://{ip}/components/cache/readable"))
            .await
            .unwrap()
            .json::<serde_json::Value>(),
        reqwest::get(format!("http://{ip}/status/powerflow"))
            .await
            .unwrap()
            .json::<serde_json::Value>()
    );
    let req = req.unwrap();
    let req1 = req1.unwrap();
    let req2 = req2.unwrap();
    let batman1 = req["Body"]["Data"]["16580608"]["channels"].clone();
    let batman: Option<FroniusBatterManagementReadableChannels> =
        serde_json::from_value(batman1).unwrap();
    let cache: Option<FroniusCacheReadableChannels> =
        serde_json::from_value(req1["Body"]["Data"]["393216"]["channels"].clone()).unwrap();
    let powerflow2: Option<FroniusPowerflow2ReadableBody> =
        serde_json::from_value(req2["inverters"][0].clone()).unwrap();
    let powerflow: Option<FroniusPowerflowReadableBody> =
        serde_json::from_value(req2["site"].clone()).unwrap();

    Metrics {
        batman,
        cache: cache.unwrap(),
        powerflow: powerflow.unwrap(),
        powerflow2: powerflow2.unwrap(),
    }
}

fn format_metrics(m: &Metrics, name: &str) -> String {
    let labels = [("name", name)];
    format!(
        "{}{}{}{}",
        serde_prometheus::to_string(&m.batman, None, &labels).unwrap(),
        serde_prometheus::to_string(&m.cache, None, &labels).unwrap(),
        serde_prometheus::to_string(&m.powerflow, None, &labels).unwrap(),
        serde_prometheus::to_string(&m.powerflow2, None, &labels).unwrap()
    )
}

async fn health() -> &'static str {
    "OK"
}

#[axum::debug_handler]
#[instrument]
async fn metrics(addrs: State<tokio::sync::watch::Receiver<Vec<SocketAddr>>>) -> String {
    let dat = addrs.borrow().clone();

    if dat.is_empty() {
        tracing::warn!("No inverters discovered yet, returning empty metrics");
        return String::new();
    }

    let results: Vec<Metrics> = join_all(dat.iter().map(|a| scrape(*a))).await;
    format!(
        "{}{}",
        results
            .iter()
            .find(|m| m.batman.is_some())
            .map(|m| format_metrics(m, "lefty"))
            .unwrap_or("".to_owned()),
        results
            .iter()
            .find(|m| m.batman.is_none())
            .map(|m| format_metrics(m, "righty"))
            .unwrap_or("".to_owned()),
    )
}
