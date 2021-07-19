#[cfg(test)]
mod tests {
    use peregrine_db::PeregrineDB;
    use lazy_static::lazy_static;
    use std::collections::HashMap;

    lazy_static! {
        static ref DB: PeregrineDB = PeregrineDB::new();
    }

    // allows testing of async code using tokio-test
    macro_rules! awt { 
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
        // writing to DB should return the result as well
        let mut hash = HashMap::new();
        hash.insert("hello".to_string(), "world".to_string());
        let res = awt!(DB.write(hash));
        let mut expected = HashMap::new();
        expected.insert("hello".to_string(), "world".to_string());
        // write hello:world, expect to get hello:world as a confirmation
        assert_eq!(res.unwrap(), expected);
    }
    
    #[test]
    fn read_test() {
        // first write to DB
        let mut expected = HashMap::new();
        expected.insert("hello".to_string(), "world".to_string());
        let mut hash = HashMap::new();
        hash.insert("hello".to_string(), "world".to_string());
        let res = awt!(DB.write(hash));
        // ensure write response matches what we sent.
        assert_eq!(res.unwrap(), expected);
        // then read from DB
        let mut select = vec![];
        select.push("hello".to_string());
        let res = awt!(DB.read(select));
        // assert that we expect the write and read to be the same.
        assert_eq!(res.unwrap(), expected);
    }
}
