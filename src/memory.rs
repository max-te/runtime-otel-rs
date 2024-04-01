use opentelemetry::metrics::{CallbackRegistration, Meter, MetricsError, ObservableGauge};

use crate::otel_ext::{ObserverExt, MeterBuilderExt};

pub fn register(meter: &Meter) -> Result<Box<(dyn CallbackRegistration)>, MetricsError> {
    let phys_mem: ObservableGauge<_> = meter
        .instrument("process.physical_memory.bytes")
        .init();
    let virt_mem: ObservableGauge<_> = meter
        .instrument("process.virtual_memory.bytes")
        .init();

    meter
        .register_callback(&[phys_mem.as_any(), virt_mem.as_any()], move |cb| {
            let Some(usage) = memory_stats::memory_stats() else {
                return;
            };
            cb.observe(&phys_mem, usage.physical_mem as u64, &[]);
            cb.observe(&virt_mem, usage.virtual_mem as u64, &[]);
        })
}
