//! keys encapsulates key management
//!
//! Processes the keys in the `keys/accounts` folder by
//! 1. Read the keys_db.yml file
//! 2. Faults in keys from file system as needed

use {
    super::load_keys_config_file,
    lazy_static::lazy_static,
    serde::{Deserialize, Serialize},
    solana_sdk::signature::{read_keypair_file, Keypair},
    std::{
        collections::HashMap,
        error, fs,
        path::{Path, PathBuf},
    },
};

/// The base folder for the keys DB
const KEYS_DB_CONFIG_PATH: &str = "keys";
/// The configuration file name for the keys in the DB
const KEYS_DB_CONFIG_FILE_NAME: &str = "keys_db.yml";
/// Standardized wallet key name
const WALLET: &str = "wallet";
/// Standardized account key name
const ACCOUNT: &str = "account";
/// The base folder for the program
const KEY_PROGRAM_PATH: &str = "program";
/// Our fee receiving account owner
const SERVICE_OWNER: &str = "Service";

// Initialize the pathbuf for the path/filename of keys db configuration
lazy_static! {
    static ref KEYS_CONFIG_PATH: PathBuf = {
        let path = Path::new(KEYS_DB_CONFIG_PATH);
        path.join(KEYS_DB_CONFIG_FILE_NAME)
    };
}

// Initialize the yaml keys hashmap
lazy_static! {
    static ref KEYS_YAML_DB: KeysYamlDB = KeysYamlDB::load();
}

// Initialize the pathbuf for the path/subpath for the program key
lazy_static! {
    static ref PROGRAM_KEY_PATH: PathBuf = {
        let path = Path::new(KEYS_DB_CONFIG_PATH);
        path.join(KEY_PROGRAM_PATH)
    };
}

// Initialize the Program Key
lazy_static! {
    pub static ref PROG_KEY: Keypair = load_program_key();
}

/// Load the programs key for deployment or usage in transactions
fn load_program_key() -> Keypair {
    if PROGRAM_KEY_PATH.is_dir() {
        let mut fpath = fs::read_dir(PROGRAM_KEY_PATH.as_path()).unwrap();
        let entry = fpath.next().unwrap().unwrap();
        match read_keypair_file(entry.path()) {
            Ok(f) => f,
            Err(_) => panic!(),
        }
    } else {
        panic!()
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
/// Encapsulates the users and keypair keypaths, internal only
struct KeysYamlDB {
    version: String,
    registry: HashMap<String, HashMap<String, PathBuf>>, // Maps from friendly "User" down to keypair paths
}

impl KeysYamlDB {
    fn load() -> Self {
        match load_keys_config_file(KEYS_CONFIG_PATH.as_path()) {
            Ok(t) => t,
            Err(_) => {
                eprintln!(
                    "{:?} not found during keys_db configuration load",
                    *KEYS_CONFIG_PATH
                );
                panic!()
            }
        }
    }

    /// Returns the configuration file map
    pub fn registry(&self) -> &HashMap<String, HashMap<String, PathBuf>> {
        &self.registry
    }

    /// Returns the configuration file version
    #[allow(dead_code)]
    pub fn version(&self) -> &String {
        &self.version
    }
}

// Initialize the keypairs hashmap
lazy_static! {
    pub static ref KEYS_DB: KeysDB = KeysDB::load();
}

#[derive(Debug)]
/// Encapsulates the users and keypairs
pub struct KeysDB {
    keys_registry: HashMap<String, HashMap<String, Keypair>>,
}

impl KeysDB {
    /// Load a account file into a Keypair
    #[allow(dead_code)]
    fn load_keypair(path: &Path) -> Result<Keypair, Box<dyn error::Error>> {
        match read_keypair_file(&path) {
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("could not read keypair file \"{}\". Run \"solana-keygen new\" to create a keypair file: {}",
                path.display(), e),
        )
        .into()),
        Ok(kp) => Ok(kp),
    }
    }

    /// Loads the KEYS_DB keypairs from the configuration
    #[allow(dead_code)]
    fn load() -> Self {
        let mut keys_reg = HashMap::<String, HashMap<String, Keypair>>::new();
        for accounts in KEYS_YAML_DB.registry().iter() {
            let mut keys_hm = HashMap::<String, Keypair>::new();
            keys_hm.insert(
                WALLET.to_string().to_string(),
                Self::load_keypair(accounts.1.get(WALLET).unwrap()).unwrap(),
            );
            keys_hm.insert(
                ACCOUNT.to_string().to_string(),
                Self::load_keypair(accounts.1.get(ACCOUNT).unwrap()).unwrap(),
            );
            keys_reg.insert(accounts.0.clone(), keys_hm);
        }
        KeysDB {
            keys_registry: keys_reg,
        }
    }
    /// Fetch a reference to the registry of keypairs
    pub fn keys_registry(&self) -> &HashMap<String, HashMap<String, Keypair>> {
        &self.keys_registry
    }
    /// Returns a vector of key owners
    pub fn key_owners(&self) -> Vec<String> {
        let mut result = Vec::<String>::new();
        for x in self.keys_registry.keys() {
            result.push(x.to_string());
        }
        result
    }
    /// Returns non service account owner names
    pub fn non_service_key_owners(&self) -> Vec<String> {
        let mut all_owners = self.key_owners();
        all_owners.remove(
            all_owners
                .iter()
                .position(|x| *x == SERVICE_OWNER)
                .expect("needle not found"),
        );
        all_owners
    }
    /// Get a wallet and account keypair for name
    pub fn wallet_and_account(
        &self,
        name: String,
    ) -> Result<(&Keypair, &Keypair), Box<dyn error::Error>> {
        match self.keys_registry.contains_key(&name) {
            true => {
                let owner = self.keys_registry.get(&name).unwrap();
                Ok((owner.get(WALLET).unwrap(), owner.get(ACCOUNT).unwrap()))
            }
            false => Err(Box::<dyn error::Error>::from(format!(
                "could not find owner \"{}\". key in DB",
                name
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use solana_sdk::signer::Signer;

    use super::*;

    #[test]
    fn test_program_key() {
        println!("{}", PROG_KEY.pubkey());
    }
    #[test]
    fn test_keys_config_db_load() {
        assert_eq!("1.5.0", KEYS_YAML_DB.version());
    }

    #[test]
    fn test_keys_keypair_load() {
        assert!(KEYS_DB.keys_registry().contains_key(SERVICE_OWNER));
        assert!(KEYS_DB.keys_registry().contains_key("User1"));
        assert!(KEYS_DB.keys_registry().contains_key("User2"));
        if let Some(user) = KEYS_DB.keys_registry().get(SERVICE_OWNER) {
            assert!(user.contains_key(WALLET));
            assert!(user.contains_key(ACCOUNT));
        }
        if let Some(user) = KEYS_DB.keys_registry().get("User1") {
            assert!(user.contains_key(WALLET));
            assert!(user.contains_key(ACCOUNT));
        }
        if let Some(user) = KEYS_DB.keys_registry().get("User2") {
            assert!(user.contains_key(WALLET));
            assert!(user.contains_key(ACCOUNT));
        }
    }

    #[test]
    fn test_list_key_holders() {
        let key_owners = KEYS_DB.key_owners();
        assert!(key_owners.contains(&"Service".to_string()));
        assert!(key_owners.contains(&"User1".to_string()));
        assert!(key_owners.contains(&"User2".to_string()));
    }

    #[test]
    fn test_non_service_key_holders() {
        let key_owners = KEYS_DB.non_service_key_owners();
        assert!(!key_owners.contains(&"Service".to_string()));
        assert!(key_owners.contains(&"User1".to_string()));
        assert!(key_owners.contains(&"User2".to_string()));
    }
}
