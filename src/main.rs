use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use axum::routing::{get, post};

async fn handler(counter: Arc<AtomicU64>) {
    let prev = counter.fetch_add(1, Ordering::SeqCst);
    println!("{prev}");
}

#[tokio::main]
async fn main() {
    let counter = Arc::new(AtomicU64::new(0));

    let make_handler = || {
        let counter = counter.clone();
        move || handler(counter)
    };

    let app = axum::Router::new()
        .route("/", get(make_handler()))
        .route("/*all", get(make_handler()))
        .route("/", post(make_handler()))
        .route("/*all", post(make_handler()));

    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
