#[macro_use] extern crate failure;

use std::collections::HashMap;
use std::path::Path;

pub mod err;

pub use err::Result;

/// Main struct containing a HashMap value.
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Returns an instance of KvStore struct.
    pub fn new() -> Result<KvStore> {
        Ok(KvStore { map: HashMap::new(), })
    }

    /// Search for key value and returns Option<String>
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(key.as_str()).cloned())
    }

    /// If not set before, creates value associated with key in KvStore.map
    /// If already set, updates that value.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key, value);
        Ok(())
    }

    /// Removes (key, value) pair of KvStore.map
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.map.remove(key.as_str());
        Ok(())
    }

    pub fn open(path: &Path) -> Result<KvStore> {
        panic!();
    }

}
