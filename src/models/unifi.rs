use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub remember_me: bool,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiEnvelope<T> {
    pub meta: UnifiEnvelopeMetadata,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiEnvelopeMetadata {
    pub rc: String,
}
