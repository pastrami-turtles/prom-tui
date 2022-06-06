# prom-tui

A simple terminal ui application to visualize Prometheus metrics.

## Usage

Start with 'cargo run' and quit by pressing 'q'.

You can provide the endpoint to scrape in 2 ways:
  1. as CLI argument
  2. as env variable

In the case of the CLI argument run:

```bash
cargo run -- --endpoint "http://localhost:8081/metrics"
```

with the env variable
```bash
PROMTUI_ENDPOINT=http://localhost:8081/metrics cargo run
```

If no endpoint is provided the default value is http://localhost:8080/metrics

## Local development

Start any component exposing a Prometheus endpoint or use the provided script '.devcontainer/serve-metrics.sh' to expose some example metrics.
