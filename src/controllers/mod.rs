use axum::{
    extract::{Path, State},
    routing::{get, put},
    Json, Router,
};
use serde_json::Value;

use crate::{
    error::AppError,
    models::{MullvadRelay, SetVpnServerRequest},
    services::{mullvad::MullvadService, unifi::UnifiService},
};

#[derive(Clone)]
pub struct AppState {
    pub mullvad_service: MullvadService,
    pub unifi_service: UnifiService,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/mullvad/servers", get(get_servers))
        .route("/api/mullvad/server/:hostname", get(get_server))
        .route("/api/unifi/vpn-clients", get(get_vpn_clients))
        .route("/api/unifi/vpn-client/:id", get(get_vpn_client))
        .route("/api/unifi/vpn-client/:id", put(set_vpn_client_server))
        .with_state(state)
}

async fn get_servers(State(state): State<AppState>) -> Result<Json<Vec<MullvadRelay>>, AppError> {
    let servers = state.mullvad_service.get_server_list().await?;
    Ok(Json(servers))
}

async fn get_server(
    State(state): State<AppState>,
    Path(hostname): Path<String>,
) -> Result<Json<MullvadRelay>, AppError> {
    let server = state.mullvad_service.get_server(hostname).await?;
    Ok(Json(server))
}

async fn get_vpn_clients(State(state): State<AppState>) -> Result<Json<Vec<Value>>, AppError> {
    let clients = state.unifi_service.get_network_configs().await?;
    Ok(Json(clients))
}

async fn get_vpn_client(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    let client = state.unifi_service.get_network_config(&id).await?;
    Ok(Json(client))
}

async fn set_vpn_client_server(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<SetVpnServerRequest>,
) -> Result<(), AppError> {
    state
        .unifi_service
        .set_vpn_client_server(&id, &payload.server_name)
        .await?;
    Ok(())
}
