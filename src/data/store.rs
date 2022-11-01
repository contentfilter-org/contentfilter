#[path="../config/mod.rs"]
mod config;

use std::fs;
use std::env;
use std::fmt;
use image::EncodableLayout;
use sqlite;
use std::path::PathBuf;
use actix_web::web::Bytes;
use rand::{distributions::Alphanumeric, Rng};


const FOREST_DBNAME: &str = "__FOREST__.sqlite";


#[derive(Debug)]
pub struct SieveAddedError;

impl fmt::Display for SieveAddedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "can not add sieve to filter!")
    }
}

fn get_forest_dbpath() -> PathBuf {
    let db_folder: PathBuf = PathBuf::from(env::var(config::DB_FOLDER).unwrap());
    let forest_dbpath: PathBuf = db_folder.join(FOREST_DBNAME);
    forest_dbpath
}

fn get_filter_dbpath(filter_name: &String) -> PathBuf {
    let db_folder: PathBuf = PathBuf::from(env::var(config::DB_FOLDER).unwrap());
    let filter_dbpath: PathBuf = db_folder.join(format!("{:}.sqlite", filter_name));
    filter_dbpath
}

pub fn init_forestdb() {
    let forest_dbpath = get_forest_dbpath();
    let con = sqlite::open(&forest_dbpath).unwrap();
    con.execute(
        "
            CREATE TABLE IF NOT EXISTS filter (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                filter_type TEXT NOT NULL,
                filter_name TEXT NOT NULL,
                labels TEXT NOT NULL,
                create_time BIGINT NOT NULL
            );
        "
    ).unwrap();
}

fn init_filterdb(filter_name: &String) {
    let filter_dbptah = get_filter_dbpath(filter_name);
    let con = sqlite::open(&filter_dbptah).unwrap();
    con.execute(
        "
            CREATE TABLE IF NOT EXISTS sieve (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                target TEXT NOT NULL,
                dr_md5 TEXT NOT NULL,
                dr_dhash TEXT NOT NULL,
                property_map TEXT NOT NULL,
                create_time BIGINT NOT NULL,
                UNIQUE(dr_md5)
            );
        "
    ).unwrap();
}

pub fn add_filter(filter_type: &String, filter_name: &String, labels: &Vec<String>) -> bool {
    init_forestdb();
    let filter_dbpath = get_filter_dbpath(filter_name);
    if filter_dbpath.exists() {
        return false;
    }
    let forest_dbpath = get_forest_dbpath();
    let connection = sqlite::open(&forest_dbpath).unwrap();
    connection.execute(
        format!(
            "
                INSERT INTO filter (filter_type, filter_name, labels, create_time) 
                VALUES ('{:}', '{:}', '{:}', strftime('%s', 'now') * 1000)
            ", filter_type, filter_name.replace("'", "''"), labels.join(";")
        )
    ).unwrap();
    init_filterdb(filter_name);
    true
}

pub fn read_filters() -> Vec<(u64, String, String, Vec<String>, u64)> {
    let mut filters: Vec<(u64, String, String, Vec<String>, u64)> = Vec::new();
    let forest_dbpath = get_forest_dbpath();
    let connection = sqlite::open(&forest_dbpath).unwrap();
    let mut cursor = connection.prepare(
        "SELECT id, filter_type, filter_name, labels, create_time FROM filter"
    ).unwrap().into_cursor();
    while let Some(Ok(row)) = cursor.next() {
        filters.push(
            (
                row.get::<i64, _>(0) as u64,
                row.get::<String, _>(1),
                row.get::<String, _>(2),
                row.get::<String, _>(3).split(";").map(|s| s.to_string()).collect::<Vec<String>>(),
                row.get::<i64, _>(4) as u64
            )
        );
    }
    filters
}

pub fn add_sieve(filter_name: &String, target: &String, dr_md5: &String, dr_dhash: u64, property_map: &String) -> Result<(u64, u64), SieveAddedError> {
    let filter_dbpath = get_filter_dbpath(filter_name);
    if !filter_dbpath.exists() {
        return Err(SieveAddedError{});
    }
    let connection = sqlite::open(&filter_dbpath).unwrap();
    connection.execute(
        format!(
                "
                    INSERT OR IGNORE INTO sieve (target, dr_md5, dr_dhash, property_map, create_time)
                    VALUES ('{}', '{}', '{}', '{}', strftime('%s', 'now') * 1000);
                ",
                target.replace("'", "''"), dr_md5, dr_dhash.to_string(), property_map
        )
    ).unwrap();
    let mut cursor = connection.prepare(
        format!("SELECT id, create_time FROM sieve WHERE dr_md5 = {:?}", dr_md5)
    ).unwrap().into_cursor();
    let mut result_tuple: (u64, u64) = (0u64, 0u64);
    if let Some(Ok(row)) = cursor.next() {
        result_tuple = (
            row.get::<i64, _>(0) as u64, 
            row.get::<i64, _>(1) as u64
        );
    }
    Ok(result_tuple)
}

pub fn read_sieves(filter_name: &String) -> Vec<(u64, String, String, u64, String, u64)> {
    let mut sieves: Vec<(u64, String, String, u64, String, u64)> = Vec::new();
    let filter_dbpath = get_filter_dbpath(filter_name);
    let connection = sqlite::open(&filter_dbpath).unwrap();
    let mut cursor = connection.prepare(
        "SELECT id, target, dr_md5, dr_dhash, property_map, create_time FROM sieve"
    ).unwrap().into_cursor();
    while let Some(Ok(row)) = cursor.next() {
        sieves.push(
            (
                row.get::<i64, _>(0) as u64,
                row.get::<String, _>(1),
                row.get::<String, _>(2),
                row.get::<String, _>(3).parse::<u64>().unwrap(),
                row.get::<String, _>(4),
                row.get::<i64, _>(5) as u64
            )
        );
    }
    sieves
}

fn keygen() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect()
}

pub fn upload_blobfile(file: Bytes) -> String {
    let key = loop {
        let new_key = keygen();
        let blob_folder: PathBuf = PathBuf::from(env::var(config::BLOB_FOLDER).unwrap());
        let file_path = blob_folder.join(&new_key);
        if !file_path.exists() {
            break new_key;
        }
    };
    let blob_folder: PathBuf = PathBuf::from(env::var(config::BLOB_FOLDER).unwrap());
    let file_path = blob_folder.join(&key);
    let status = fs::write(file_path, file.as_bytes());
    if let Ok(_) = status {
        return key;
    }
    "".to_string()
}

