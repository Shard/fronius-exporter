# Fronius Exporter Helm Chart
Deploys fronius exporter into Kubernetes using Helm.

## Prerequisites
- Kubernetes cluster
- Helm 3+ installed

## Installing the Chart
Add the repository and update:

```shell
helm repo add fronius-exporter https://ghcr.io/shard/charts/
helm repo update
```

To install the chart with the release name `fronius-exporter`:
```shell
helm install fronius-exporter fronius-exporter/fronius-exporter
```

## Configuration Values
The following table lists the configurable parameters for the fronius-exporter chart:

| Parameter | Description | Default |
|-----------|-------------|---------|
| `image.repository` | Image repository | `ghcr.io/yourusername/fronius-exporter` |
| `image.tag` | Image tag | `latest` |
| `image.pullPolicy` | Image pull policy | `IfNotPresent` |
| `serviceAccount.create` | Create a service account | `true` |
| `service.type` | Kubernetes service type | `ClusterIP` |
| `service.port` | Service port | `8000` |
| `env.DEFAULT_NETWORK` | Network CIDR for inverter discovery | `192.168.1.0/24` |
| `env.RUST_LOG` | Log level | `debug` |
| `resources.limits.cpu` | CPU limit | `100m` |
| `resources.limits.memory` | Memory limit | `128Mi` |
| `resources.requests.cpu` | CPU request | `50m` |
| `resources.requests.memory` | Memory request | `64Mi` |
| `prometheus.enabled` | Deploy Prometheus alongside | `false` |
| `grafana.enabled` | Deploy Grafana alongside | `false` |

### Custom Values
To override default values, create a `values.yaml` file and pass it during installation:

```shell
helm install fronius-exporter fronius-exporter/fronius-exporter -f values.yaml
```

Example `values.yaml`:

```yaml
replicaCount: 2
env:
  DEFAULT_NETWORK: "10.0.0.0/24"
  RUST_LOG: "info"
resources:
  limits:
    cpu: 200m
    memory: 256Mi
```

### Deploying with Prometheus and Grafana
You can also deploy with a built in prometheus and grafana pre-setup if you want to quickly demo it:

```yaml
prometheus:
  enabled: true

grafana:
  enabled: true
```

This will deploy Prometheus and Grafana instances pre-configured to collect and visualize metrics from your Fronius inverters.
