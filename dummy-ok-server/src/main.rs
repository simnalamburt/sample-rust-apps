use axum::routing::{get, post};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::{Duration, Instant};

static COUNTER: AtomicU64 = AtomicU64::new(0);

async fn handler() {
    COUNTER.fetch_add(1, Ordering::SeqCst);
}

#[tokio::main]
async fn main() {
    thread::spawn(move || {
        let mut prev = Instant::now();
        println!("rate\tcount\telapsed seconds");
        loop {
            thread::sleep(Duration::from_millis(250));

            let count = COUNTER.swap(0, Ordering::SeqCst);

            let now = Instant::now();
            let elapsed = now.duration_since(prev).as_secs_f64();
            let speed = count as f64 / elapsed;
            println!("{speed}\t{count}\t{elapsed}");

            prev = now;
        }
    });

    let app = axum::Router::new()
        .route("/", get(handler))
        .route("/*all", get(handler))
        .route("/", post(handler))
        .route("/*all", post(handler));

    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
