pub mod mullvad;
pub mod unifi;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ErrorResponse {
    pub r#type: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MullvadRelay {
    pub hostname: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub location: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SetVpnServerRequest {
    pub server_name: String,
}
