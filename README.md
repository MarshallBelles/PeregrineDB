# PeregrineDB
Peregrine DB is an in-memory thread-safe data store library for Rust.

## Usage: 
In cargo.toml:
```
[dependencies]
peregrine_db = {git = "https://github.com/MarshallBelles/PeregrineDB"}
```

In .rs:
```RS
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

```

Output:
```
user@localhost example % cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/example`
[2021-07-19T19:49:21Z INFO  example] PeregrineDB initialized
[2021-07-19T19:49:21Z INFO  example] Saved data
[2021-07-19T19:49:21Z INFO  example] Read data
[2021-07-19T19:49:21Z INFO  example] {"hello": "world"} == {"hello": "world"}
```
