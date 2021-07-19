use peregrine_db::PeregrineDB;
use std::collections::HashMap;
use futures::executor::block_on;

#[macro_use]
extern crate log;

async fn peregrine_example() {
    // initialize new DB
    let db: PeregrineDB = PeregrineDB::new();
    info!("PeregrineDB initialized");

    // save some data
    let mut data = HashMap::new();
    data.insert("hello".to_string(), "world".to_string());
    let response = db.write(data).await.unwrap();
    info!("Saved data");

    // read the data
    let mut keys = vec![];
    keys.push("hello".to_string());
    let stored_data = db.read(keys).await.unwrap();
    info!("Read data");

    // write and read should be the same
    assert_eq!(stored_data, response);
    info!("{:?} == {:?}", stored_data, response);

}

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"));
    block_on(peregrine_example());
}
