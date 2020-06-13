#[cfg(test)]
mod tests {
    use peregrine_db::PeregrineDB;
    use lazy_static::lazy_static;
    use std::sync::Mutex;
    use std::collections::HashMap;

    lazy_static! {
        static ref DB: PeregrineDB = PeregrineDB::new();
    }

    // allows testing of async code using tokio-test
    macro_rules! aw { 
        ($e:expr) => {
            tokio_test::block_on($e)
        };
      }    

    #[test]
    fn setup_test() {
        DB.start();
    }

    #[test]
    fn write_test() {
        let mut hash = HashMap::new();
        hash.insert("hello".to_string(), "world".to_string());
        let res = aw!(DB.write(hash));
        let mut expected = HashMap::new();
        expected.insert("hello".to_string(), "world".to_string());
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn read_test() {
        let mut select = vec![];
        select.push("hello".to_string());
        let res = aw!(DB.read(select));
        let mut expected = HashMap::new();
        expected.insert("hello".to_string(), "world".to_string());
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    fn persistence_test() {
        aw!(DB.save());
        let new_test_db = PeregrineDB::new();
        let mut select = vec![];
        select.push("hello".to_string());
        let res = aw!(new_test_db.read(select));
        let mut expected = HashMap::new();
        expected.insert("hello".to_string(), "world".to_string());
        assert_eq!(res.unwrap(), expected);
    }
}
