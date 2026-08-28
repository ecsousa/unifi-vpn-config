use crate::config::AppConfig;
use crate::error::AppError;
use crate::models::unifi::{LoginRequest, UnifiEnvelope};
use crate::services::mullvad::MullvadService;
use moka::future::Cache;
use reqwest::{Client, cookie::Jar};
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct UnifiService {
    config: AppConfig,
    client: Client,
    mullvad_service: MullvadService,
    cookie_jar: Arc<Jar>,
    csrf_cache: Cache<String, String>,
}

impl UnifiService {
    pub fn new(config: AppConfig, mullvad_service: MullvadService) -> Self {
        let cookie_jar = Arc::new(Jar::default());
        let client = Client::builder()
            .cookie_provider(cookie_jar.clone())
            .timeout(Duration::from_secs(10))
            .danger_accept_invalid_certs(true) // Unifi often uses self-signed
            .build()
            .unwrap();

        let csrf_cache = Cache::builder()
            .time_to_live(Duration::from_secs(5 * 60))
            .build();

        Self {
            config,
            client,
            mullvad_service,
            cookie_jar,
            csrf_cache,
        }
    }

    async fn login(&self) -> Result<String, AppError> {
        if let Some(csrf) = self.csrf_cache.get("csrf").await {
            return Ok(csrf);
        }

        let login_url = format!("{}/api/auth/login", self.config.unifi_base_url);
        let req_body = LoginRequest {
            username: self.config.unifi_username.clone(),
            password: self.config.unifi_password.clone(),
            remember_me: false,
            token: "".to_string(),
        };

        let resp = self.client.post(&login_url).json(&req_body).send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(AppError::LoginFailed {
                provider: "unifi".to_string(),
                status_code: status.as_u16(),
                message: body,
            });
        }

        let csrf = resp
            .headers()
            .get("X-CSRF-Token")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default()
            .to_string();

        self.csrf_cache.insert("csrf".to_string(), csrf.clone()).await;

        Ok(csrf)
    }

    async fn get_network_conf_url(&self, id: Option<&str>) -> String {
        format!(
            "{}/proxy/network/api/s/default/rest/networkconf/{}",
            self.config.unifi_base_url,
            id.unwrap_or("")
        )
    }

    pub async fn get_network_configs(&self) -> Result<Vec<Value>, AppError> {
        let csrf = self.login().await?;
        let url = self.get_network_conf_url(None).await;

        let mut req = self.client.get(&url);
        if !csrf.is_empty() {
            req = req.header("X-CSRF-Token", &csrf);
        }

        let env: UnifiEnvelope<Value> = req.send().await?.json().await?;
        Ok(env.data)
    }

    pub async fn get_network_config(&self, id: &str) -> Result<Value, AppError> {
        let csrf = self.login().await?;
        let url = self.get_network_conf_url(Some(id)).await;

        let mut req = self.client.get(&url);
        if !csrf.is_empty() {
            req = req.header("X-CSRF-Token", &csrf);
        }

        let env: UnifiEnvelope<Value> = req.send().await?.json().await?;
        env.data.into_iter().next().ok_or_else(|| AppError::ResourceNotFound {
            resource_type: "networkConfig".to_string(),
            resource_id: id.to_string(),
        })
    }

    pub async fn set_vpn_client_server(&self, id: &str, server_name: &str) -> Result<(), AppError> {
        let relay = self.mullvad_service.get_server(server_name.to_string()).await?;
        let mut node = self.get_network_config(id).await?;

        if let Some(obj) = node.as_object_mut() {
            obj.insert("wireguard_client_peer_ip".to_string(), json!(relay.hostname));
            obj.insert("wireguard_client_peer_public_key".to_string(), json!(relay.public_key));
        }

        let csrf = self.login().await?;
        let url = self.get_network_conf_url(Some(id)).await;

        let mut req = self.client.put(&url).json(&node);
        if !csrf.is_empty() {
            req = req.header("X-CSRF-Token", &csrf);
        }

        let resp = req.send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
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
