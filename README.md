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

Prereqs:
* VS Code (incl. extentions)
  * ms-vscode-remote.remote-containers
* Docker

Using the 'Remote - Containers' extension, the command 'Reopen in Container' will open the project within the configured container environment. This also starts a second container which exposes Prometheus metrics on http://localhost:8080/metrics.
