use ethers::prelude::{Address, H256, U256};
use eyre::Result;
use serde::de::Error;
use serde::Deserialize;

use crate::common::utils::hex_str_to_bytes;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Proof {
    pub address: Address,
    pub balance: U256,
    pub code_hash: H256,
    pub nonce: U256,
    pub storage_hash: H256,
    #[serde(deserialize_with = "proof_deserialize")]
    pub account_proof: Vec<Vec<u8>>,
}

fn proof_deserialize<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let branch: Vec<String> = serde::Deserialize::deserialize(deserializer)?;
    Ok(branch
        .iter()
        .map(|elem| hex_str_to_bytes(elem))
        .collect::<Result<_>>()
        .map_err(D::Error::custom)?)
}
