#[cfg(feature = "memory-stats")]
pub mod memory;
mod otel_ext;

#[cfg(all(tokio_unstable, feature = "tokio"))]
pub mod tokio_rt;
#[cfg(all(not(tokio_unstable), feature = "tokio"))]
pub mod tokio_rt {
    use opentelemetry::metrics::{
        noop::NoopRegistration, CallbackRegistration, Meter, MetricsError,
    };
    use tokio::runtime::Handle;

    pub fn register_tokio_metrics(
        _runtime: Handle,
        _meter: &Meter,
    ) -> Result<Box<dyn CallbackRegistration>, MetricsError> {
        Ok(Box::new(NoopRegistration::new()))
    }
}
