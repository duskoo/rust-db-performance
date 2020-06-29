#![feature(test)]
extern crate test;

#[allow(soft_unstable)]
// #![cfg_attr(test, feature(test))]

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use std::path::Path;
    use std::str;
    use persy::{Persy, PersyError, PersyId, Value, ValueMode};

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }

    #[bench]
    fn persy_save(b: &mut Bencher){
        //https://gitlab.com/tglman/persy/-/blob/master/examples/indexes.rs
        let create_segment;
        if !Path::new("index.exp").exists() {
            Persy::create("index.exp").expect("DB creation failed!");
            create_segment = true;
        } else {
            create_segment = false;
        }
    
        let persy : Persy = Persy::open("index.exp", persy::Config::new()).expect("open");
        if create_segment {
            let mut tx = persy.begin().expect("Transaction failed to start");
            tx.create_segment("data").expect("segment");
            tx.create_index::<String, PersyId>("index", ValueMode::REPLACE).expect("index");
            let prepared = tx.prepare_commit().expect("prepare");
            prepared.commit().expect("commit");
        }
        b.iter(|| save_one_read_one(&persy));

    }

    fn save_one_read_one(persy: &Persy){
        let mut tx = persy.begin().expect("begin");
        let rec = "aaaa".as_bytes();
        let id = tx.insert_record("data", rec).expect("insert");
    
        tx.put::<String, PersyId>("index", "key".to_string(), id).expect("put");
        let prepared = tx.prepare_commit().expect("prep");
        prepared.commit().expect("c2");

        let read_id = persy.get::<String, PersyId>("index", &"key".to_string()).expect("read id");
        if let Some(is_there) = read_id {
            if let Value::SINGLE(id) = is_there {
                let value = persy.read_record("data", &id).expect("read");
                assert_eq!(Some(rec.to_vec()), value);
            }
        }
    }

    #[test]
    pub fn persy_examine(){
        let persy : Persy = Persy::open("index.exp", persy::Config::new()).expect("open");

        let items = persy
            .scan("data").expect("scan error");
            let mut cnt = 1;
            for (_id, content) in items {
                println!("{}.", cnt);
                cnt += 1;
                println!("_id: {}", _id);
                println!("content: {:#?}", content);
                let st = str::from_utf8(&content).unwrap();
                println!("string : {}", st);
            }
            // println!("num items: {}", items.len());
    }

}


fn main() {
    println!("Hello, world!");
}


