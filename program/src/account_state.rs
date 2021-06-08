//! account_state manages account data

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use borsh::{BorshDeserialize, BorshSerialize};
use crate::error::SampleError;
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
};
use std::collections::BTreeMap;

/// Initialization flag size for account state
const INITIALIZED_BYTES: usize = 1;
/// Storage for the serialized size of the BTreeMap control
const BTREE_LENGTH: usize = 4;
/// Storage for the serialized BTreeMap container
const BTREE_STORAGE: usize = 1019;
/// Sum of all account state lengths
const ACCOUNT_STATE_SPACE: usize = INITIALIZED_BYTES + BTREE_LENGTH + BTREE_STORAGE;

/// Maintains global accumulator
#[derive(Debug, Default, PartialEq)]
pub struct ProgramAccountState {
    is_initialized: bool,
    btree_storage: BTreeMap<String, String>,
}

impl ProgramAccountState {
    /// Adds a new key/value pair to the account
    pub fn add(&mut self, key: String, value: String) -> Result<(), SampleError> {
        match self.btree_storage.contains_key(&key) {
            true => Err(SampleError::KeyAlreadyExists),
            false => {
                self.btree_storage.insert(key, value);
                Ok(())
            },
        }
    }
    /// Removes a key from account and returns the keys value
    pub fn remove(&mut self, key: &str) -> Result<String, SampleError> {
        match self.btree_storage.contains_key(key) {
            true => Ok(self.btree_storage.remove(key).unwrap()),
            false => Err(SampleError::KeyNotFoundInAccount),
        }
    }
}

impl Sealed for ProgramAccountState {}

impl IsInitialized for ProgramAccountState {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

#[allow(clippy::ptr_offset_with_cast)]
impl Pack for ProgramAccountState {
    const LEN: usize = ACCOUNT_STATE_SPACE;

    /// Store 'state' of account to its data area
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, ACCOUNT_STATE_SPACE];
        // Setup pointers to key areas of account state data
        let (is_initialized_dst, data_len_dst, data_dst) =
            mut_array_refs![dst, INITIALIZED_BYTES, BTREE_LENGTH, BTREE_STORAGE];
        // Set the initialized flag
        is_initialized_dst[0] = self.is_initialized as u8;
        // Store the core data length and serialized content
        let keyval_store_data = self.btree_storage.try_to_vec().unwrap();
        let data_len = keyval_store_data.len();
        data_len_dst[..].copy_from_slice(&(data_len as u32).to_le_bytes());
        data_dst[0..data_len].copy_from_slice(&keyval_store_data);
        }

    /// Retrieve 'state' of account from account data area
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, ACCOUNT_STATE_SPACE];
        // Setup pointers to key areas of account state data
        let (is_initialized_src, data_len_src, data_src) =
            array_refs![src, INITIALIZED_BYTES, BTREE_LENGTH, BTREE_STORAGE];

        let is_initialized = match is_initialized_src {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        // Get current size of content in data area
        let data_len = u32::from_le_bytes(*data_len_src) as usize;
        // If emptry, create a default
        if data_len == 0 {
            Ok(ProgramAccountState {
                is_initialized,
                btree_storage: BTreeMap::<String, String>::new(),
            })
        } else {
            let data_dser =
                BTreeMap::<String, String>::try_from_slice(&data_src[0..data_len]).unwrap();
            Ok(ProgramAccountState {
                is_initialized,
                btree_storage: data_dser,
            })
        }
    }
}
