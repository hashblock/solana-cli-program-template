//! utils

use std::{fs::File, io, path::Path};

pub mod account_state;
pub mod account_utils;
pub mod keys_db;

/// Instructions that program usess
pub enum Instructions {
    InitializeAccount = 0,
    FreeMint = 1,
    FreeTransfer = 2,
    FreeBurn = 3,
}

/// Loads a yaml file
pub fn load_keys_config_file<T, P>(config_file: P) -> Result<T, io::Error>
where
    T: serde::de::DeserializeOwned,
    P: AsRef<Path>,
{
    let file = File::open(config_file)?;
    let config = serde_yaml::from_reader(file)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("{:?}", err)))?;
    Ok(config)
}
