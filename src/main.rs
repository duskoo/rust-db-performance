#![feature(test)]
extern crate test;

#[allow(soft_unstable)]
// #![cfg_attr(test, feature(test))]

mod helpers;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use persy::{Persy, PersyError, PersyId, Value, ValueMode};
    use std::path::Path;
    use std::str;
    use test::Bencher;
    use crate::helpers::create_and_drop;
    use std::time::Instant;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }

    // #[bench]
    fn persy_save(b: &mut Bencher) {
        let persy = persy_setup();
        b.iter(|| save_one_read_one(&persy));
    }
    #[bench]
    fn persy_load(b: &mut Bencher) {
        let persy = persy_setup();
        b.iter(|| persy_examine(&persy));
    }

    fn persy_setup() -> Persy {
        //https://gitlab.com/tglman/persy/-/blob/master/examples/indexes.rs
        let create_segment;
        if !Path::new("index.exp").exists() {
            Persy::create("index.exp").expect("DB creation failed!");
            create_segment = true;
        } else {
            create_segment = false;
        }

        let persy: Persy = Persy::open("index.exp", persy::Config::new()).expect("open");
        if create_segment {
            let mut tx = persy.begin().expect("Transaction failed to start");
            tx.create_segment("data").expect("segment");
            tx.create_index::<String, PersyId>("index", ValueMode::REPLACE)
                .expect("index");
            let prepared = tx.prepare_commit().expect("prepare");
            prepared.commit().expect("commit");
        }

        persy
    }

    fn save_one_read_one(persy: &Persy) {
        let mut tx = persy.begin().expect("begin");
        let rec = "aaaa".as_bytes();
        let id = tx.insert_record("data", rec).expect("insert");

        tx.put::<String, PersyId>("index", "key".to_string(), id)
            .expect("put");
        let prepared = tx.prepare_commit().expect("prep");
        prepared.commit().expect("c2");

        let read_id = persy
            .get::<String, PersyId>("index", &"key".to_string())
            .expect("read id");
        if let Some(is_there) = read_id {
            if let Value::SINGLE(id) = is_there {
                let value = persy.read_record("data", &id).expect("read");
                assert_eq!(Some(rec.to_vec()), value);
            }
        }
    }

    // #[test]
    pub fn persy_examine(persy: &Persy) {
        // let persy : Persy = Persy::open("index.exp", persy::Config::new()).expect("open");

        let items = persy.scan("data").expect("scan error");
        let mut cnt = 0;
        for (_id, content) in items {
            cnt += 1;
            let st = str::from_utf8(&content).unwrap();
            println!("{}.", cnt);
            println!("string : {}", st);
            if false {
                println!("_id: {}", _id);
                println!("content: {:#?}", content);
            }
        }
        // println!("num items: {}", items.len());
    }

    #[test]
    //https://gitlab.com/tglman/persy/-/blob/master/tests/record_operations.rs#L180
    fn test_insert_100_same_tx() {
        /*
         use std::time::Instant;
    let now = Instant::now();

    {
        my_function_to_measure();
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2}", elapsed);
        */
        let now = Instant::now();
        create_and_drop("i100", |persy| {
            let mut tx = persy.begin().unwrap();
            tx.create_segment("test").unwrap();
            let bytes = String::from("something").into_bytes();
            for _ in [0; 100].iter() {
                tx.insert_record("test", &bytes).unwrap();
            }
            let finalizer = tx.prepare_commit().unwrap();
            finalizer.commit().unwrap();
    
            let mut count = 0;
            for _ in persy.scan("test").unwrap() {
                count += 1;
            }
            assert_eq!(count, 100);
        });

        let elapsed = now.elapsed();
        println!("Elapsed: {:#?}", elapsed);
    }
    
    
}

fn main() {
    println!("Hello, world!");
}
