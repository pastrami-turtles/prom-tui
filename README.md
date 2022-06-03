# prom-tui

A simple terminal ui application to visualize Prometheus metrics.

## Usage

Start with 'cargo run' and quit by pressing 'q'.

## Local development

Start any local component exposing a Prometheus endpoint.

```sh
$ nerdctl run --name=redpanda-1 --rm \
    -p 9644:9644 \
    docker.redpanda.com/vectorized/redpanda:latest \
    redpanda start \
    --overprovisioned \
    --smp 1  \
    --memory 1G \
    --reserve-memory 0M \
    --node-id 0 \
    --check=false
```