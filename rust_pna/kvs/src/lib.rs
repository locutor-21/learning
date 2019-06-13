use std::collections::HashMap;

<<<<<<< HEAD
// DRAGONBALL
=======
/// Main struct containing a HashMap value.
#[derive(Default)]
>>>>>>> finished project1 for pna course
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
<<<<<<< HEAD
    pub fn new() -> Self {
        let mut map = HashMap::new();
        Self {
            map     
        }
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        let mut opt = self.map.get(key.as_str());
        if opt.is_some(){
            Some(opt.unwrap().to_owned())
        } else{
=======
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
>>>>>>> finished project1 for pna course
            None
        }
    }

<<<<<<< HEAD
=======
    /// If not set before, creates value associated with key in KvStore.map
    /// If already set, updates that value.
>>>>>>> finished project1 for pna course
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

<<<<<<< HEAD
=======
    /// Removes (key, value) pair of KvStore.map
>>>>>>> finished project1 for pna course
    pub fn remove(&mut self, key: String) {
        self.map.remove(key.as_str());
    }
}
