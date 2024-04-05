use std::time::Duration;
use tracing::{info, instrument, span, Level};
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, layer::SubscriberExt};

fn main() {
    let _guard = init_tracing();
    start_processes();
}

fn start_processes() {
    let span = span!(Level::TRACE, "start");
    let _guard = span.enter();

    let mut threads = Vec::new();
    for process in std::env::args() {
        let span = span.clone(); // cloning here in order to move the cloned spawn to another thread and have it register within it
        threads.push(std::thread::spawn(move || {
            span.in_scope(move || do_something(process));
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
}

#[instrument]
fn do_something(process_name: String) {
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
