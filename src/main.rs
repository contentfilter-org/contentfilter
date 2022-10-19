mod config;
mod filter;
// mod service;

use std::sync::Mutex;
use actix_web::{web::Data, App, HttpServer};

use actix_web::{
    web::{Bytes},
    post, HttpResponse,
};
// use std::sync::Mutex;
use json::JsonValue;
use filter::forest::FilterForest;



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

#[post("/filter/create")]
pub async fn create(forest: Data<Mutex<FilterForest>>, body: Bytes) -> HttpResponse {
    let result = json::parse(std::str::from_utf8(&body).unwrap());
    let injson: JsonValue = match result {
        Ok(req) => {
            let filter_type = req["filter_type"].as_str().unwrap().to_string();
            let filter_name = req["filter_name"].as_str().unwrap().to_string();
            let mut labels: Vec<String> = Vec::new();
            for label in req["labels"].members(){
                labels.push(label.clone().take().to_string());
            }
            forest.lock().unwrap().add_filter(&filter_type, &filter_name, &labels);
            json::object!{
                "success": true
            }
        },
        Err(e) => json::object! {
            "err": e.to_string(),
            "success": false
        },
    };
    HttpResponse::Ok()
    .content_type("application/json")
    .body(injson.dump()) 
}

#[post("/sieve/add")]
pub async fn add(forest: Data<Mutex<FilterForest>>, body: Bytes) -> HttpResponse {
    let result = json::parse(std::str::from_utf8(&body).unwrap());
    let injson: JsonValue = match result {
        Ok(req) => {
            let filter_name = req["filter_name"].as_str().unwrap().to_string();
            let target = req["target"].as_str().unwrap().to_string();
            let property_map = req["property_map"].to_string();
            forest.lock().unwrap().add_sieve(&filter_name, &target, &property_map);
            json::object!{
                "success": true
            }
        },
        Err(e) => json::object! {
            "err": e.to_string(),
            "success": false
        },
    };
    HttpResponse::Ok()
    .content_type("application/json")
    .body(injson.dump())
}

#[post("/detect")]
pub async fn detect(forest: Data<Mutex<FilterForest>>, body: Bytes) -> HttpResponse {
    let result = json::parse(std::str::from_utf8(&body).unwrap());
    let injson: JsonValue = match result {
        Ok(req) => {
            let filter_name = req["filter_name"].as_str().unwrap().to_string();
            let content = req["content"].as_str().unwrap().to_string();
            let resp = forest.lock().unwrap().detect(&filter_name, &content);
            resp.unwrap()
        },
        Err(e) => json::object! {
            "err": e.to_string(),
            "success": false
        },
    };
    HttpResponse::Ok()
    .content_type("application/json")
    .body(injson.dump())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init_config();
    let forest = Data::new(Mutex::new(filter::forest::FilterForest::new()));
    print_hello();
    HttpServer::new(move || {
        App::new()
        .app_data(forest.clone())
        .service(create)
        .service(add)
        .service(detect)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
