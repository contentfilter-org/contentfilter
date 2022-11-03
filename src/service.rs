use serde::Deserialize;


#[derive(PartialEq)]
#[allow(dead_code)]
pub enum ServiceStatus {
    Ok,
    RequestParameterError,
    FilterNotFoundError,
    FilterExistsError,
    FilterTypeNotFoundError,
    SieveNotFoundError,
    SieveAddError,
    InvalidImageError,
    FileNotFoundError,
    FileUploadError
}

impl std::fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ServiceStatus::Ok => write!(f, "ok"),
            ServiceStatus::RequestParameterError => write!(f, "error: wrong request parameter"),
            ServiceStatus::FilterNotFoundError => write!(f, "error: filter not found"),
            ServiceStatus::FilterExistsError => write!(f, "error: filter exists"),
            ServiceStatus::FilterTypeNotFoundError => write!(f, "error: filter type not found"),
            ServiceStatus::SieveNotFoundError => write!(f, "error: sieve not found"),
            ServiceStatus::SieveAddError => write!(f, "error: can not add sieve"),
            ServiceStatus::InvalidImageError => write!(f, "error: invalid image"),
            ServiceStatus::FileNotFoundError => write!(f, "error: file not found"),
            ServiceStatus::FileUploadError => write!(f, "error: can not upload file")
        }
    }
}

pub fn print_hello() {
    let hello = r"
         _   _        _  _             _____                _                 _    ______  _  _  _                _ 
        | | | |      | || |           /  __ \              | |               | |   |  ___|(_)| || |              | |
        | |_| |  ___ | || |  ___      | /  \/  ___   _ __  | |_   ___  _ __  | |_  | |_    _ | || |_   ___  _ __ | |
        |  _  | / _ \| || | / _ \     | |     / _ \ | '_ \ | __| / _ \| '_ \ | __| |  _|  | || || __| / _ \| '__|| |
        | | | ||  __/| || || (_) | _  | \__/\| (_) || | | || |_ |  __/| | | || |_  | |    | || || |_ |  __/| |   |_|
        \_| |_/ \___||_||_| \___/ ( )  \____/ \___/ |_| |_| \__| \___||_| |_| \__| \_|    |_||_| \__| \___||_|   (_)
    ";
    println!("{}", hello)
}

#[derive(Deserialize, Debug)]
pub struct FilterCreateRequest {
    pub filter_type: String,
    pub filter_name: String,
    pub labels: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct SieveAddRequest {
    pub filter_name: String,
    pub target: String,
    pub property_map: std::collections::HashMap<String, String>
}

#[derive(Deserialize, Debug)]
pub struct DetectRequest {
    pub filter_name: String,
    pub content: String
}
