use axum::{
    Router,
    routing::{post, get},
    AddExtensionLayer,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;

mod root;
mod health;
mod todo;
mod db;
mod domain;

#[tokio::main]
async fn main() {
    let db = db::DB::default();

    let app = Router::new()
        .route("/", get(root::root))
        .route("/todos", post(todo::create))
        .route("/health", get(health::health))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(db))
        );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
