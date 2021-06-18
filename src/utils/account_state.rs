//! @brief Account state access

use {
    crate::utils::txn_utils::get_account_for,
    arrayref::*,
    borsh::BorshDeserialize,
    solana_client::rpc_client::RpcClient,
    solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair, signer::Signer},
    std::{collections::BTreeMap, error::Error},
};

/// Initialization flag size for account state
const INITIALIZED_BYTES: usize = 1;
/// Storage for the serialized size of the BTreeMap control
const BTREE_LENGTH: usize = 4;
/// Storage for the serialized BTreeMap container
const BTREE_STORAGE: usize = 1019;
/// Sum of all account state lengths
pub const ACCOUNT_STATE_SPACE: usize = INITIALIZED_BYTES + BTREE_LENGTH + BTREE_STORAGE;

/// Unpacks the token state and returns serialized accumulator value
#[allow(clippy::clippy::ptr_offset_with_cast)]
fn unpack_from_slice(src: &[u8]) -> Result<(bool, BTreeMap<String, String>), Box<dyn Error>> {
    let src = array_ref![src, 0, ACCOUNT_STATE_SPACE];
    // Setup pointers to key areas of account state data
    let (is_initialized_src, data_len_src, data_src) =
        array_refs![src, INITIALIZED_BYTES, BTREE_LENGTH, BTREE_STORAGE];

    let is_initialized = match is_initialized_src {
        [0] => false,
        [1] => true,
        _ => {
            return Err(Box::<dyn Error>::from(format!(
                "unrecognized initialization flag \"{:?}\". in account",
                is_initialized_src
            )))
        }
    };
    // Get current size of content in data area
    let data_len = u32::from_le_bytes(*data_len_src) as usize;
    // If emptry, create a default
    if data_len == 0 {
        Ok((is_initialized, BTreeMap::<String, String>::new()))
    } else {
        let data_dser = BTreeMap::<String, String>::try_from_slice(&data_src[0..data_len]).unwrap();
        Ok((is_initialized, data_dser))
    }
}

/// Unpacks token state for the accumulator
pub fn unpack_account_data(
    rpc_client: &RpcClient,
    account: &Keypair,
    commitment_config: CommitmentConfig,
) -> Result<(bool, BTreeMap<String, String>), Box<dyn Error>> {
    match get_account_for(rpc_client, &account.pubkey(), commitment_config) {
        Some(account_) => Ok(unpack_from_slice(&account_.data).unwrap()),
        None => Err(Box::<dyn Error>::from(format!(
            "account not found for \"{:?}\". ",
            account
        ))),
    }
}
