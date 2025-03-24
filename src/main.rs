use eyre::Result;
use std::net::{Ipv4Addr, SocketAddr};
use tracing_subscriber::EnvFilter;

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

async fn get_servers() -> tokio::sync::watch::Receiver<Vec<SocketAddr>> {
    // Define the subnet (CIDR format)
    let cidr = std::env::var("DEFAULT_NETWORK").unwrap();

    // Parse the network
    let network: Ipv4Network = cidr.parse().expect("Invalid CIDR format");

    // Get the starting IP address of the network and the number of addresses in the subnet
    let start_ip = network.network().clone();
    let end_ip = network.broadcast().clone();

    let (send, recv) = tokio::sync::watch::channel(vec![]);

    tokio::task::spawn(async move {
        // Iterate over all the IP addresses in the subnet
        let results: Vec<SocketAddr> = join_all(
            ip_range(start_ip, end_ip)
                .filter(|ip| *ip != network.network() && *ip != network.broadcast())
                .map(|ip_addr| {
                    reqwest::get(format!(
                        "http://{ip_addr}/components/BatteryManagementSystem/readable",
                    ))
                }),
        )
        .await
        .iter()
        .filter_map(|resp| {
            let Ok(resp) = resp else {
                return None;
            };
            if resp.status() != 200 {
                return None;
            }
            if resp.headers().get(HeaderName::from_static("content-type"))
                != Some(&HeaderValue::from_static("text/javascript"))
            {
                return None;
            }
            resp.remote_addr()
        })
        .collect();
        send.send(results).unwrap();
        sleep(std::time::Duration::from_secs(60 * 5)).await;
    });

    recv
}

#[derive(Clone)]
struct MetricsState {
    addresses: Vec<SocketAddr>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    println!("🚀 Fronius-exporter is starting..");
    tracing::event!(tracing::Level::DEBUG, "Finding inverters");
    let addrs = get_servers().await;
    tracing::event!(tracing::Level::DEBUG, "Found inverters: {:?}", &addrs);
    let app = axum::Router::new()
        .route("/metrics", get(metrics))
        .with_state(addrs);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    tracing::event!(tracing::Level::DEBUG, "Starting metrics endpoint");
    axum::serve(listener, app).await.unwrap();
}

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

#[axum::debug_handler]
async fn metrics(addrs: State<tokio::sync::watch::Receiver<Vec<SocketAddr>>>) -> String {
    let dat = addrs.borrow().clone();
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
