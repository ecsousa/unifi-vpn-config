use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MullvadRelaysResponse {
    pub wireguard: MullvadWireguard,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MullvadWireguard {
    pub relays: Vec<MullvadWireguardRelay>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MullvadWireguardRelay {
    pub hostname: String,
    pub public_key: String,
    pub location: String,
}
