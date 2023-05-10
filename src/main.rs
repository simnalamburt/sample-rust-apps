use axum::routing::{get, post};

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(|| async { "" }))
        .route("/*all", get(|| async { "" }))
        .route("/", post(|| async { "" }))
        .route("/*all", post(|| async { "" }));

    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
