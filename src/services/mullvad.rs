use crate::error::AppError;
use crate::models::{mullvad::MullvadRelaysResponse, MullvadRelay};
use moka::future::Cache;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct MullvadService {
    client: Client,
    cache: Cache<String, Arc<MullvadRelaysResponse>>,
}

impl MullvadService {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        let cache = Cache::builder()
            .time_to_live(Duration::from_secs(10 * 60))
            .build();

        Self { client, cache }
    }

    async fn fetch_relays(&self) -> Result<Arc<MullvadRelaysResponse>, AppError> {
        if let Some(cached) = self.cache.get("relays").await {
            return Ok(cached);
        }

        let resp: MullvadRelaysResponse = self
            .client
            .get("https://api.mullvad.net/app/v1/relays")
            .send()
            .await?
            .json()
            .await?;

        let arc_resp = Arc::new(resp);
        self.cache
            .insert("relays".to_string(), arc_resp.clone())
            .await;
        Ok(arc_resp)
    }

    pub async fn get_server_list(&self) -> Result<Vec<MullvadRelay>, AppError> {
        let relays = self.fetch_relays().await?;

        Ok(relays
            .wireguard
            .relays
            .iter()
            .map(|r| MullvadRelay {
                hostname: format!("{}.relays.mullvad.net", r.hostname),
                public_key: r.public_key.clone(),
                location: r.location.clone(),
            })
            .collect())
    }

    pub async fn get_server(&self, name: String) -> Result<MullvadRelay, AppError> {
        let relays = self.fetch_relays().await?;

        relays
            .wireguard
            .relays
            .iter()
            .find(|r| r.hostname == name)
            .map(|r| MullvadRelay {
                hostname: format!("{}.relays.mullvad.net", r.hostname),
                public_key: r.public_key.clone(),
                location: r.location.clone(),
            })
            .ok_or_else(|| AppError::ResourceNotFound {
                resource_type: "mullvadRelay".to_string(),
                resource_id: name,
            })
    }
}
