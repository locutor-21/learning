use std::collections::HashMap;

/// Main struct containing a HashMap value.
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Returns an instance of KvStore struct.
    pub fn new() -> Self {
        let map = HashMap::new();
        Self { map }
    }

    /// Search for key value and returns Option<String>
    pub fn get(&mut self, key: String) -> Option<String> {
        let opt = self.map.get(key.as_str());
        if opt.is_some() {
            Some(opt.unwrap().to_owned())
        } else {
            None
        }
    }

    /// If not set before, creates value associated with key in KvStore.map
    /// If already set, updates that value.
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Removes (key, value) pair of KvStore.map
    pub fn remove(&mut self, key: String) {
        self.map.remove(key.as_str());
    }
}
