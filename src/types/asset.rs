use std::collections::BTreeMap;

use alloy_primitives::Address;

use super::NetworkId;

#[derive(Debug)]
pub enum AssetAddress {
    Eip155(Address),
    Native,
}

#[derive(Debug)]
pub enum AssetType {
    Crypto,
    Fiat,
}

#[derive(Debug)]
pub struct Asset {
    pub symbol: String,
    pub asset_id: String,
    pub name: String,
    pub ty_: AssetType,
    pub crypto_address: BTreeMap<NetworkId, AssetAddress>,
}
