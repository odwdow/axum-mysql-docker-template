pub mod context;
pub mod adopter;
pub mod model;
pub mod service;
pub mod ui;

use std::env;
use std::net::{Ipv4Addr, SocketAddr};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const PORT_NUMBER: u16 = 8888;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "codetest=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = ui::router().await;
    let host = get_host();
    let addr = SocketAddr::from((host, PORT_NUMBER));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_host() -> Ipv4Addr {
    let ip_str = env::var("HOST").unwrap_or(String::from("127.0.0.1"));

    ip_str.parse().unwrap()
}
