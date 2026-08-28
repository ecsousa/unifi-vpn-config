mod config;
mod controllers;
mod error;
mod models;
mod services;

use std::net::SocketAddr;

use crate::{
    config::AppConfig,
    controllers::{create_router, AppState},
    services::{mullvad::MullvadService, unifi::UnifiService},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = AppConfig::from_env();
    let port = config.port;
    
    let mullvad_service = MullvadService::new();
    let unifi_service = UnifiService::new(config, mullvad_service.clone());

    let state = AppState {
        mullvad_service,
        unifi_service,
    };

    let app = create_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
