use std::borrow::Cow;

use opentelemetry::metrics::{
    AsyncInstrument, AsyncInstrumentBuilder, Meter, ObservableCounter, ObservableGauge,
    Observer,
};
use opentelemetry::KeyValue;

pub trait MeterBuilderExt<I, M>
where
    I: AsyncInstrument<M>,
{
    fn instrument(
        &self,
        name: impl Into<Cow<'static, str>>,
    ) -> AsyncInstrumentBuilder<'_, I, M>;
}

impl MeterBuilderExt<ObservableCounter<u64>, u64> for Meter {
    fn instrument(
        &self,
        name: impl Into<Cow<'static, str>>,
    ) -> AsyncInstrumentBuilder<'_, ObservableCounter<u64>, u64> {
        self.u64_observable_counter(name)
    }
}

impl MeterBuilderExt<ObservableCounter<f64>, f64> for Meter {
    fn instrument(
        &self,
        name: impl Into<Cow<'static, str>>,
    ) -> AsyncInstrumentBuilder<'_, ObservableCounter<f64>, f64> {
        self.f64_observable_counter(name)
    }
}

impl MeterBuilderExt<ObservableGauge<u64>, u64> for Meter {
    fn instrument(
        &self,
        name: impl Into<Cow<'static, str>>,
    ) -> AsyncInstrumentBuilder<'_, ObservableGauge<u64>, u64> {
        self.u64_observable_gauge(name)
    }
}

impl MeterBuilderExt<ObservableGauge<f64>, f64> for Meter {
    fn instrument(
        &self,
        name: impl Into<Cow<'static, str>>,
    ) -> AsyncInstrumentBuilder<'_, ObservableGauge<f64>, f64> {
        self.f64_observable_gauge(name)
    }
}

pub trait ObserverExt<O> {
    fn observe(&self, inst: &dyn AsyncInstrument<O>, measurement: O, attrs: &[KeyValue]);
}

impl ObserverExt<f64> for &dyn Observer {
    fn observe(&self, inst: &dyn AsyncInstrument<f64>, measurement: f64, attrs: &[KeyValue]) {
        self.observe_f64(inst, measurement, attrs);
    }
}

impl ObserverExt<u64> for &dyn Observer {
    fn observe(&self, inst: &dyn AsyncInstrument<u64>, measurement: u64, attrs: &[KeyValue]) {
        self.observe_u64(inst, measurement, attrs);
    }
}

impl ObserverExt<i64> for &dyn Observer {
    fn observe(&self, inst: &dyn AsyncInstrument<i64>, measurement: i64, attrs: &[KeyValue]) {
        self.observe_i64(inst, measurement, attrs);
    }
}
