mod request;
mod response;
mod view;

use std::net::SocketAddr;

use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use view::root;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "sunrise-api=warn,tower_http=warn".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build application
    let app = Router::new()
        .route("/", get(root))
        .layer(TraceLayer::new_for_http());

    // Run application
    let addr: SocketAddr = std::env::var("BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3000".into())
        .parse()
        .unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
