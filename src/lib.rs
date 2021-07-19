use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use snafu::Snafu;

pub struct PeregrineDB (pub Arc<RwLock<HashMap<String, String>>>);

impl PeregrineDB {
    pub fn new() -> PeregrineDB {
        let hm: HashMap<String, String> = HashMap::new();
        let db = PeregrineDB(Arc::new(RwLock::new(hm)));
        return db;
    }
    pub fn start(&self) {
        // database init for future use
    }
    pub async fn read(&self, keys: Vec<String>) -> Result<HashMap<String, String>, PeregrineError> {
        // borrow the HashMap sharingly
        let read_write = Arc::clone(&self.0);
        let hash_map = read_write.read().unwrap();
        let mut response: HashMap<String, String> = HashMap::new();
        for query in keys {
            if hash_map.contains_key(&query) {
                response.insert(query.clone(), hash_map.get(&query).unwrap().to_string());
            } else {
                response.insert(query.clone(), String::from(""));
            }
        }
        Ok(response)
    }
    pub async fn write(&self, map: HashMap<String, String>) -> Result<HashMap<String, String>, PeregrineError> {
        // exclusively lease the HashMap
        let read_write = Arc::clone(&self.0);
        let mut hash_map = read_write.write().unwrap();
        for (key, val) in map.iter() {
            hash_map.entry(key.to_string()).and_modify(|e| { *e = val.to_string() }).or_insert(val.to_string());
        }
        Ok(map)
    }
}

#[derive(Debug, Snafu)]
pub enum PeregrineError {
    #[snafu(display("Unspecified Error"))]
    Unspecified
    // future implementation of error catching here - there really shouldn't be any.
}

type Result<T, E = PeregrineError> = std::result::Result<T, E>;