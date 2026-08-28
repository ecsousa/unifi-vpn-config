use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub unifi_username: String,
    pub unifi_password: String,
    pub unifi_base_url: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let unifi_username = env::var("UNIFI_USERNAME").unwrap_or_default();
        let unifi_password = env::var("UNIFI_PASSWORD").unwrap_or_default();
        let unifi_base_url = env::var("UNIFI_BASEURL").unwrap_or_default();
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);

        Self {
            unifi_username,
            unifi_password,
            unifi_base_url,
            port,
        }
    }
}
