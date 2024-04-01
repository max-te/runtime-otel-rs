mod otel_ext;
#[cfg(feature = "memory-stats")]
pub mod memory;

#[cfg(all(tokio_unstable, feature = "tokio"))]
pub mod tokio_rt;
#[cfg(all(not(tokio_unstable), feature = "tokio"))]
pub mod tokio_rt {
    use tokio::runtime::Handle;
    use opentelemetry::metrics::{CallbackRegistration, noop::NoopRegistration, Meter, MetricsError};

    pub fn register_tokio_metrics(_runtime: Handle, _meter: &Meter) -> Result<Box<dyn CallbackRegistration>, MetricsError> {
        Ok(Box::new(NoopRegistration::new()))
    }
}
