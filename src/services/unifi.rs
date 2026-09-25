use crate::config::AppConfig;
use crate::error::AppError;
use crate::models::unifi::UnifiEnvelope;
use crate::services::mullvad::MullvadService;
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;

#[derive(Clone)]
pub struct UnifiService {
    config: AppConfig,
    client: Client,
    mullvad_service: MullvadService,
}

impl UnifiService {
    pub fn new(config: AppConfig, mullvad_service: MullvadService) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        Self {
            config,
            client,
            mullvad_service,
        }
    }

    async fn get_network_conf_url(&self, id: Option<&str>) -> String {
        format!(
            "{}/proxy/network/api/s/default/rest/networkconf/{}",
            self.config.unifi_base_url,
            id.unwrap_or("")
        )
    }

    pub async fn get_network_configs(&self) -> Result<Vec<Value>, AppError> {
        let url = self.get_network_conf_url(None).await;

        let mut req = self.client.get(&url);
        req = req.header("X-API-KEY", &self.config.unifi_apikey);

        let resp = req.send().await?;
        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::UnifiUnauthorized);
        } else if resp.status() == reqwest::StatusCode::FORBIDDEN {
            return Err(AppError::UnifiForbidden);
        }

        let env: UnifiEnvelope<Value> = resp.json().await?;
        Ok(env.data)
    }

    pub async fn get_network_config(&self, id: &str) -> Result<Value, AppError> {
        let url = self.get_network_conf_url(Some(id)).await;

        let mut req = self.client.get(&url);
        req = req.header("X-API-KEY", &self.config.unifi_apikey);

        let resp = req.send().await?;
        if resp.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err(AppError::UnifiUnauthorized);
        } else if resp.status() == reqwest::StatusCode::FORBIDDEN {
            return Err(AppError::UnifiForbidden);
        }

        let env: UnifiEnvelope<Value> = resp.json().await?;
        env.data
            .into_iter()
            .next()
            .ok_or_else(|| AppError::ResourceNotFound {
                resource_type: "networkConfig".to_string(),
                resource_id: id.to_string(),
            })
    }

    pub async fn set_vpn_client_server(&self, id: &str, server_name: &str) -> Result<(), AppError> {
        let relay = self
            .mullvad_service
            .get_server(server_name.to_string())
            .await?;
        let mut node = self.get_network_config(id).await?;

        if let Some(obj) = node.as_object_mut() {
            obj.insert(
                "wireguard_client_peer_ip".to_string(),
                json!(relay.hostname),
            );
            obj.insert(
                "wireguard_client_peer_public_key".to_string(),
                json!(relay.public_key),
            );
        }

        let url = self.get_network_conf_url(Some(id)).await;

        let mut req = self.client.put(&url).json(&node);
        req = req.header("X-API-KEY", &self.config.unifi_apikey);

        let resp = req.send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            if status == reqwest::StatusCode::UNAUTHORIZED {
                return Err(AppError::UnifiUnauthorized);
            } else if status == reqwest::StatusCode::FORBIDDEN {
                return Err(AppError::UnifiForbidden);
            }

            let body = resp.text().await.unwrap_or_default();
            tracing::error!("Failed to update VPN client server (id: {}): {}", id, body);
            return Err(AppError::InternalError(format!(
                "Failed to update VPN client server: HTTP {} {}",
                status, body
            )));
        }

        Ok(())
    }
}
