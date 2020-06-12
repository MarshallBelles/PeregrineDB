use std::sync::{Arc, RwLock};
use std::thread;
use std::collections::HashMap;
use std::any::Any;
use snafu::{ResultExt, Snafu};
use std::{fs, io, path::PathBuf};

pub struct PeregrineDB (Arc<RwLock<HashMap<String, String>>>);

impl PeregrineDB {
    fn new() -> PeregrineDB {
        return load().expect("Could not create the database");
    }
    fn start(&self) {
        // database init
    }
    async fn read(&self, keys: Vec<String>) -> Result<HashMap<String, String>, PeregrineError> {
        // borrow the HashMap sharingly
        let RW = Arc::clone(&self.0);
        let HM = RW.read().unwrap();
        let mut response: HashMap<String, String> = HashMap::new();
        for query in keys {
            if HM.contains_key(&query) {
                response.insert(query.clone(), HM.get(&query).unwrap().to_string());
            } else {
                response.insert(query.clone(), String::from(""));
            }
        }
        Ok(response)
    }
    async fn write(&self, map: HashMap<String, String>) -> Result<HashMap<String, String>, PeregrineError> {
        // exclusively lease the HashMap
        let RW = Arc::clone(&self.0);
        let mut HM = RW.write().unwrap();
        for (key, val) in map.iter() {
            HM.entry(key.to_string()).and_modify(|e| { *e = val.to_string() }).or_insert(val.to_string());
        }
        Ok(map)
    }
}

#[derive(Debug, Snafu)]
pub enum PeregrineError {
    #[snafu(display("Unable to read database from {}: {}", path.display(), source))]
    ReadDB{ source: io::Error, path: PathBuf },

    #[snafu(display("Unable to write database to {}: {}", path.display(), source))]
    WriteDB { source: io::Error, path: PathBuf },

    #[snafu(display("Unspecified Error"))]
    Unspecified,
}

type Result<T, E = PeregrineError> = std::result::Result<T, E>;

pub fn save(db: &'static PeregrineDB) -> Result<(), PeregrineError> {
    // save to disk async - Move this to it's own thread
    let writer = thread::spawn(move || -> Result<(), PeregrineError> {
        write_to_disk(db)?;
        Ok(())
    });
    let result = writer.join().unwrap()?;
    Ok(())
}

// exposed for advanced usage
pub fn load() -> Result<PeregrineDB, PeregrineError> {
    // initialize new DB structure
    let mut hm: HashMap<String, String> = HashMap::new();
    let db = PeregrineDB(Arc::new(RwLock::new(hm)));
    // read from the disk
    let reader = thread::spawn(move || -> Result<PeregrineDB, PeregrineError> {
        read_from_disk(&db)?;
        Ok(db)
    });
    let result = reader.join().unwrap()?;
    Ok(result)
}

fn read_from_disk(db: &PeregrineDB) -> Result<(), PeregrineError> {
    // read disk and write to the DB instance
    Ok(())
}

fn write_to_disk(db: &PeregrineDB) -> Result<(), PeregrineError> {
    // write to disk
    Ok(())
}