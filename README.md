# ☀️ Fronius Exporter
Allows collection of metrics from Fronius Inverters that can be collected by [Prometheus](https://grafana.com/oss/prometheus/) for a [Grafana](https://grafana.com/grafana/) Dashboard.

![image](https://github.com/user-attachments/assets/a0451e0e-782e-4d5d-9472-38a489df4ebd)


## Compatabiity
Currently only tested with a Gen 24 Primo v8.0 in a dual inveter configuration. More work will need to be done to support most setups.

## Usage
Quick start with prometheus and grafana included.
``` shell
docker compose up -d
```

Simply visit http://localhost:3000 to visit the local grafana, or http://localhost:9090 to inspect prometheus directly. The logs for fronius exporter can be checked with `docker compose logs fronius-exporter`.

## Configuration
TODO
