use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub unifi_base_url: String,
    pub unifi_apikey: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let unifi_base_url = env::var("UNIFI_BASEURL")
            .expect("UNIFI_BASEURL must be set");
        let unifi_apikey = env::var("UNIFI_APIKEY")
            .expect("UNIFI_APIKEY must be set");
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);

        Self {
            unifi_base_url,
            unifi_apikey,
            port,
        }
    }
}
