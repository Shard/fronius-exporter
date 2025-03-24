# Grafana Dashboards

Contains the JSON definitions for Grafana dashboards that can be imported directly into your Grafana instance.

## Available Dashboards

- **fronius.json**: Dashboard for monitoring Fronius inverter metrics.

## How to Import Dashboards

1. Access your Grafana instance in a web browser
2. Navigate to Dashboards (sidebar) → New → Import
3. Select one of the following options:
   - Upload JSON file: Choose the dashboard JSON file from this folder
   - Paste JSON: Copy the contents of the JSON file and paste it into the text area
4. Click "Load"
5. Review the dashboard settings (you may need to select the appropriate data source)
6. Click "Import"

## Configuration Notes

- The grafana dashbaord should be linked to the same Prometheus server that is collecting from the `fronius-exporter`.

## Exporting Your Modifications

To share your modified dashboards:
1. Navigate to the dashboard settings (gear icon)
2. Select "JSON Model"
3. Copy the JSON or use "Save to file"
4. Add the new JSON file to this folder with an appropriate name.

If you want to submit fixes/improvements or new dashboards, feel free to open a PR.
