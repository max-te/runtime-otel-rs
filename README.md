# runtime-otel

runtime-otel is an experimental crate that enables you to meter your tokio runtime and memory usage with [OpenTelemetry](https://crates.io/crates/opentelemetry).
For tokio metrics it leans on [tokio's unstable runtime metrics](https://docs.rs/tokio/latest/tokio/runtime/struct.RuntimeMetrics.html).
For memory usage we use the [memory-stats](https://crates.io/crates/memory-stats) crate.

## Usage

To use the `tokio` feature, you must compile with the rustc flag `--cfg tokio_unstable`.

```rust
// Register Tokio metrics with OpenTelemetry
runtime_otel::tokio_rt::register_tokio_metrics(
    tokio::runtime::Handle::current(),
    &opentelemetry::global::meter("tokio"),
)?;

// Register memory metrics
runtime_otel::memory::register(&opentelemetry::global::meter("memory"))?;
```
