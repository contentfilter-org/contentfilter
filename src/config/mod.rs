use std::env;
use dirs::home_dir;
use std::path::PathBuf;
use std::fs::create_dir_all;


pub const DATA_FOLDER: &str = "DATA_FOLDER";
pub const LOG_FOLDER: &str = "LOG_FOLDER";
pub const BLOB_FOLDER: &str = "BLOB_FOLDER";
pub const DB_FOLDER: &str = "DBFOLDER";


pub fn init_config() {
    let data_folder: PathBuf = home_dir().unwrap().join("contentfilter").join("data");
    create_dir_all(&data_folder).unwrap_or_else(|e| panic!("error creating dir: {}", e));
    let log_folder: PathBuf = data_folder.join("log");
    create_dir_all(&log_folder).unwrap_or_else(|e| panic!("error creating dir: {}", e));
    let db_folder: PathBuf = data_folder.join("db");
    create_dir_all(&db_folder).unwrap_or_else(|e| panic!("error creating dir: {}", e));
    let blob_folder: PathBuf = data_folder.join("blob");
    create_dir_all(&blob_folder).unwrap_or_else(|e| panic!("error creating dir: {}", e));

    env::set_var(DATA_FOLDER, data_folder);
    env::set_var(LOG_FOLDER, log_folder);
    env::set_var(DB_FOLDER, db_folder);
    env::set_var(BLOB_FOLDER, blob_folder);
}
