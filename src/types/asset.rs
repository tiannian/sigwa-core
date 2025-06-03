use std::collections::BTreeMap;

use alloy_primitives::Address;
use serde::{Deserialize, Serialize};

use super::NetworkId;

#[derive(Debug, Serialize, Deserialize)]
pub enum AssetAddress {
    Eip155(Address),
    Native,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AssetType {
    Crypto,
    Fiat,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub symbol: String,
    pub asset_id: String,
    pub name: String,
    pub ty_: AssetType,
    pub crypto_address: BTreeMap<NetworkId, AssetAddress>,
}
