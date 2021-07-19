# PeregrineDB
Peregrine DB is an in-memory thread-safe data store library for Rust.

## Usage: 
In cargo.toml:
`peregrine_db = {git = "https://github.com/MarshallBelles/PeregrineDB"}`

In .rs:
`let DB: PeregrineDB = PeregrineDB::new();`

#### Documentation is a work in progress, this is a very simple implementation of an in-memory database which is really just a thread-safe wrapper of a HashMap
