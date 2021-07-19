# PeregrineDB
Peregrine DB is an in-memory thread-safe data store library for Rust.

## Usage: 
In cargo.toml:
`peregrine_db = {git = "https://github.com/MarshallBelles/PeregrineDB"}`

In .rs:
```RS
// async helper for demonstration:
macro_rules! awt {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

// init DB
let DB: PeregrineDB = PeregrineDB::new();

// let's write to DB
let mut hash = HashMap::new();
hash.insert("hello".to_string(), "world".to_string());
let hello_world = awt!(DB.write(hash));

// then read from DB
let mut select = vec![];
select.push("hello".to_string());
let res = awt!(DB.read(select));

// we should get back what we put in.
assert_eq!(res.unwrap(), hello_world.unwrap());

```

#### Documentation is a work in progress, this is a very simple implementation of an in-memory database which is really just a thread-safe wrapper of a HashMap
