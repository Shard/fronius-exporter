# ☀️ Fronius Exporter
Allows collection of metrics from Fronius Inverters that can be collected by [Prometheus](https://grafana.com/oss/prometheus/) for a [Grafana](https://grafana.com/grafana/) Dashboard.

![image](https://github.com/user-attachments/assets/a0451e0e-782e-4d5d-9472-38a489df4ebd)

## Compatability
Currently only tested with a Gen 24 Primo v8.0 in a dual inveter configuration. If you have an inverter you would like to see supported, feel free to open a new Issue.

## Installation and Usage

### Docker
To get started quickly, you can use Docker to run fronius-exporter locally:
```
docker run --rm \
  -p 8000:8000 \
  ghcr.io/shard/fronius-exporter:latest
```
This will run just the fronius-exporter metrics endpoint on [localhost:8000](http://localhost:8000/metrics).

Configuration can be supplied with the `-e` flag, for example: `-e DEFAULT_NETWORK=192.168.1.0/24`.

### Docker Compose
Alternatively, docker compose can be used to run a prometheus and grafana server which is pre-configured for fronius-exporter with:
``` shell
git clone git@github.com:Shard/fronius-exporter.git
cd fronius-exporter
docker compose up -d
```

Simply visit http://localhost:3000 to visit the local grafana, or http://localhost:9090 to inspect prometheus directly. The logs for fronius exporter can be checked with `docker compose logs fronius-exporter`.

Configuration can be added to a `.env` file in the same folder as the `compose.yml` file using standard env format (`KEY=VALUE`).

Only the `compose.yml` file will be required, which can instead be copied and adapted into your own grafana stack compose file.

### Kubernetes Helm Chart
Fronius Exporter can be deployed on [Kubernetes](https://kubernetes.io/) using the provided [Helm](https://helm.sh/) chart. The chart is published alongside the container on GitHub Container Registry.

Check out the [charts/README.md](charts/README.md) for more information.

## Configuration Reference
- **DEFAULT_NETWORK**: Subnet (CIDR format) from which the application will discover and connect to Fronius inverters. Example: `192.168.0.0/24`.
