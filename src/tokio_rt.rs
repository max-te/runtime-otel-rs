use opentelemetry::metrics::{CallbackRegistration, Meter};
use tokio::runtime::Handle;
use opentelemetry::metrics::{MetricsError, ObservableCounter, ObservableGauge, Observer, Unit};
use opentelemetry::KeyValue;

use crate::otel_ext::{MeterBuilderExt, ObserverExt};

macro_rules! register_instruments_and_callback {
    (@register $meter:ident $scope:literal $name:ident: $type:ident$([$($key:ident = $val:literal),*])?) => {
        let $name: $type<_> = {
            #[allow(unused_mut)]
            let mut builder = $meter
            .instrument(concat!($scope, stringify!{$name}));
            $(
                $(register_instruments_and_callback!(@apply_attr builder $key = $val);)*
            )?
            builder.init()
        };
    };

    (@apply_attr $builder:ident unit = $unit:literal) => {
        $builder = $builder.with_unit(Unit::new($unit))
    };

    (@apply_attr $builder:ident description = $desc:literal) => {
        $builder = $builder.with_description($desc)
    };

    (
        $meter:ident,
        $metrics:ident = $init:expr => { $($(#$lattr:tt)? $name:ident = $typ:ident($val:expr);)*
            |$worker:ident| {
                $($(#$wlattr:tt)? $wname:ident = $wtyp:ident($wval:expr);)*
            }
        }
    ) => {
        {
            $(register_instruments_and_callback!(@register $meter "tokio.runtime." $name: $typ$($lattr)?);)*
            $(register_instruments_and_callback!(@register $meter "tokio.runtime.worker." $wname: $wtyp$($wlattr)?);)*
            $meter
                .register_callback(
                    &[
                        $($name.as_any(), )*
                        $($wname.as_any(), )*
                    ],
                    move |obs: &dyn Observer| {
                        let $metrics = $init;
                        $(obs.observe(&$name, $val, &[]);)*
                        for i in 0..$metrics.num_workers() {
                            let $worker = i;
                            let worker_label = KeyValue::new("worker", i.to_string());
                            $(obs.observe(&$wname, $wval, &[worker_label.clone()]);)*
                        }
                    }
                )
        }
    };
}

pub fn register_tokio_metrics(runtime: Handle, meter: &Meter) -> Result<Box<dyn CallbackRegistration>, MetricsError> { 
    register_instruments_and_callback!(
        meter,
        metrics = runtime.metrics() => {
            workers = ObservableGauge(metrics.num_workers() as u64);
            blocking_threads = ObservableGauge(metrics.num_blocking_threads() as u64);
            active_tasks = ObservableGauge(metrics.active_tasks_count() as u64);
            blocking_threads_idle = ObservableGauge(metrics.num_idle_blocking_threads() as u64);
            remote_schedule = ObservableGauge(metrics.remote_schedule_count());
            budget_forced_yield = ObservableCounter(metrics.budget_forced_yield_count());
            injection_queue = ObservableGauge(metrics.injection_queue_depth() as u64);
            blocking_queue = ObservableGauge(metrics.blocking_queue_depth() as u64);
            |worker| {
                park = ObservableCounter(metrics.worker_park_count(worker));
                noop = ObservableCounter(metrics.worker_noop_count(worker));
                steal = ObservableCounter(metrics.worker_steal_count(worker));
                steal_ops = ObservableCounter(metrics.worker_steal_operations(worker));
                poll = ObservableCounter(metrics.worker_poll_count(worker));
                local_schedule = ObservableCounter(metrics.worker_local_schedule_count(worker));
                overflow = ObservableCounter(metrics.worker_overflow_count(worker));
                local_queue = ObservableCounter(metrics.worker_local_queue_depth(worker) as u64);
                #[unit = "s"]
                busy = ObservableCounter(metrics.worker_total_busy_duration(worker).as_secs_f64());
                #[unit = "s"]
                poll_mean = ObservableGauge(metrics.worker_mean_poll_time(worker).as_secs_f64());
            }
        }
    )
}
