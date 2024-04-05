use std::time::Duration;
use tracing::{info, instrument, trace_span};
use tracing_appender::non_blocking::NonBlocking;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, layer::SubscriberExt};

fn main() {
    let _guard = init_tracing();
    start_process();
}

fn start_process() {
    let span = trace_span!("my_span", pid = std::process::id());
    let _enter = span.enter();
    do_something();
}

#[instrument]
fn do_something() {
    for i in 0..=10 {
        info!("processed {}%", i * 10);
        std::thread::sleep(Duration::from_millis(50))
    }
}

fn init_tracing() -> WorkerGuard {
    let (appender, guard) = get_trace_appender();
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(appender))
        .with(EnvFilter::from_default_env()) // uses the value of RUST_LOG
        .init();

    guard
}

fn get_trace_appender() -> (NonBlocking, WorkerGuard) {
    tracing_appender::non_blocking(std::io::stdout())
}
