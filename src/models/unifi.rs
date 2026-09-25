use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiEnvelope<T> {
    pub meta: UnifiEnvelopeMetadata,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnifiEnvelopeMetadata {
    pub rc: String,
}
