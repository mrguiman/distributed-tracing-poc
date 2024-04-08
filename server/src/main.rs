use std::time::Duration;
use tracing::{info, instrument, span, Instrument, Level};
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, layer::SubscriberExt};

#[tokio::main]
async fn main() {
    let _guard = init_tracing();
    start_processes().await
}

async fn start_processes() {
    let span = span!(Level::TRACE, "start");
    let _guard = span.enter();

    let mut threads = Vec::new();
    for process in std::env::args() {
        let span = span.clone();

        threads.push(tokio::spawn(async move {
            do_something(process).instrument(span).await;
        }));
    }
    for t in threads {
        t.await.unwrap();
    }
}

#[instrument]
async fn do_something(_process_name: String) {
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
