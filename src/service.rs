use serde::Deserialize;


pub enum ServiceStatus {
    Success,
    RequestParameterError,
    FilterNotFoundError,
    FilterExistsError,
    FilterTypeNotFoundError,
    SieveNotFoundError
}

impl std::fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ServiceStatus::Success => write!(f, "success"),
            ServiceStatus::RequestParameterError => write!(f, "request parameter error"),
            ServiceStatus::FilterNotFoundError => write!(f, "filter not found error"),
            ServiceStatus::FilterExistsError => write!(f, "filter exists error"),
            ServiceStatus::FilterTypeNotFoundError => write!(f, "filter type not found error"),
            ServiceStatus::SieveNotFoundError => write!(f, "sieve not found error"),
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