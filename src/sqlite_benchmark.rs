// extern crate helpers;
// use crate::helpers as db_testing;
// use super::*;

//https://github.com/rusqlite/rusqlite/blob/master/src/lib.rs#L1585
// use db_testing;//::helpers::create_and_drop;
use rusqlite::{params, Connection, Row, ToSql};
use std::time::Instant;
use tempfile::Builder;
use test::Bencher;

#[cfg(test)]

fn sqlite_setup() -> Connection {
    println!("sqlite setup");
    let sqlite_path = "sqlite1.db";
    let connection = Connection::open(sqlite_path).unwrap();
    println!("{:#?}", connection);
    // let db = Database::new(connection.unwrap());

    let res = connection.execute(
        "create table if not exists data(
                    key integer primary key autoincrement,
                    value text not null
            )",
        params![],
    );

    connection
}

fn insert(conn: &Connection) {
    let res = conn.execute(
        "insert or replace into data(value) values(?1)",
        params!["bbbb"],
    );

    println!("res: {:#?}", res);
}

#[test]
fn test_insert_100_sqlite() {
    println!("sqlite create_table_test");
    let conn = sqlite_setup();
    let now = Instant::now();
    sqlite_insert_multiple(&conn);
    let elapsed = now.elapsed();
    println!("Elapsed: {:#?}", elapsed);
}

#[test]
fn test_insert_100_sqlite_temp(){
    /*
    Elapsed: 97.746231ms
test sqlite_benchmark::test_insert_100_sqlite_temp ... ok

    Elapsed: 88.603022ms
test sqlite_benchmark::test_insert_100_sqlite_temp ... ok

    */
    let now = Instant::now();

    temp_db("i100", |conn: &Connection| {
        sqlite_insert_multiple(conn)
    });
    let elapsed = now.elapsed();
    println!("Elapsed: {:#?}", elapsed);
}

fn sqlite_insert_multiple(conn: &Connection) {
    for _ in [0; 100].iter() {
        insert(conn);
    }
}

fn sqlite_examine(conn: &Connection) {
    let mut query = conn.prepare("SELECT key, value FROM data").unwrap();
    let mut rows = query.query(params![]).unwrap();

    while let Some(row) = rows.next().unwrap() {
        let i = row.get_raw(0).as_i64().unwrap();
        // let expect = vals[i as usize];
        let x = row.get_raw(1).as_str().unwrap();
        // assert_eq!(x, expect);
        println!("{} - {}", i, x);
    }
}

#[bench]
fn bench_load_sqlite(b: &mut Bencher) {
    /*
        13603 - bbbb
    test sqlite_benchmark::bench_load_sqlite ... bench: 126,548,067 ns/iter (+/- 16,008,466)

    13603 - bbbb
test sqlite_benchmark::bench_load_sqlite ... bench: 160,975,215 ns/iter (+/- 1,085,954,137)
        */
    let conn = sqlite_setup();
    b.iter(|| sqlite_examine(&conn));
}

#[test]
fn test_load_sqlite() {
    /*
        13603 - bbbb
    Elapsed: 287.618172ms
    test sqlite_benchmark::test_load_sqlite ... ok

    13603 - bbbb
    Elapsed: 218.315774ms
    test sqlite_benchmark::test_load_sqlite ... ok
        */
    println!("sqlite test_load_sqlite");
    let conn = sqlite_setup();
    let now = Instant::now();
    sqlite_examine(&conn);
    let elapsed = now.elapsed();
    println!("Elapsed: {:#?}", elapsed);
}

#[bench]
fn bench_sqlite_save(b: &mut Bencher) {
    let conn = sqlite_setup();
    b.iter(|| insert(&conn));
}

fn temp_db<T>(name: &str, test: T)
where
    T: FnOnce(&Connection),
{
    let tmp_file = Builder::new()
        .prefix(name)
        .suffix(".sqlite")
        .tempfile()
        .expect("expect temp file creation");

    {   
        let file = tmp_file.reopen().expect("reopen");
        let conn = Connection::open(tmp_file.path()).unwrap();

        let res = conn.execute(
            "create table if not exists data(
                        key integer primary key autoincrement,
                        value text not null
                )",
            params![],
        );

        test(&conn);
    }
}
