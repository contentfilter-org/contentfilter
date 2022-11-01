mod data;
mod config;
mod filter;
mod service;

use actix_web::{
    App,
    post, 
    HttpResponse, 
    HttpServer,
    web::{Bytes, PayloadConfig, Data}
};
use data::store::upload_blobfile;
use std::sync::Mutex;
use std::time::Instant;
use filter::forest::FilterForest;
use service::{
    FilterCreateRequest, 
    SieveAddRequest, 
    DetectRequest, 
    ServiceStatus, 
    print_hello
};


#[post("/blob/upload")]
pub async fn upload(body: Bytes) -> HttpResponse {
    let start_time = Instant::now();
    let key = upload_blobfile(body);
    let rsp_obj = serde_json::json!(
        {
            "status": ServiceStatus::Ok.to_string(),
            "time": start_time.elapsed().as_secs_f64(),
            "key": key
        }
    );
    HttpResponse::Ok()
    .content_type("application/json")
    .body(rsp_obj.to_string()) 
}


#[post("/filter/create")]
pub async fn create(forest: Data<Mutex<FilterForest>>, body: Bytes) -> HttpResponse {
    let start_time = Instant::now();
    let req_body = std::str::from_utf8(&body).unwrap();
    let req_obj: Result<FilterCreateRequest, serde_json::Error> = serde_json::from_str(req_body);
    let rsp_obj = match req_obj {
        Ok(req) => {
            let op_status = forest.lock().unwrap().add_filter(&req.filter_type, &req.filter_name, &req.labels);
            serde_json::json!(
                {
                    "status": op_status.to_string(),
                    "time": start_time.elapsed().as_secs_f64()
                }
            )
        },
        Err(_e) => serde_json::json!(
            {
                "status": ServiceStatus::RequestParameterError.to_string(),
                "time": start_time.elapsed().as_secs_f64()
            }
        )
    };
    HttpResponse::Ok()
    .content_type("application/json")
    .body(rsp_obj.to_string()) 
}

#[post("/sieve/add")]
pub async fn add(forest: Data<Mutex<FilterForest>>, body: Bytes) -> HttpResponse {
    let start_time = Instant::now();
    let req_body = std::str::from_utf8(&body).unwrap();
    let req_obj: Result<SieveAddRequest, serde_json::Error> = serde_json::from_str(req_body);
    let rsp_obj = match req_obj {
        Ok(req) => {
            let property_map = serde_json::json!(req.property_map).to_string();
            let op_status = forest.lock().unwrap().add_sieve(&req.filter_name, &req.target, &property_map);
            serde_json::json!(
                {
                    "status": op_status.to_string(),
                    "time": start_time.elapsed().as_secs_f64()
                }
            )
        },
        Err(_e) => serde_json::json!(
            {
                "status": ServiceStatus::RequestParameterError.to_string(),
                "time": start_time.elapsed().as_secs_f64()
            }
        )
    };
    HttpResponse::Ok()
    .content_type("application/json")
    .body(rsp_obj.to_string())
}

#[post("/detect")]
pub async fn detect(forest: Data<Mutex<FilterForest>>, body: Bytes) -> HttpResponse {
    let start_time = Instant::now();
    let req_body = std::str::from_utf8(&body).unwrap();
    let req_obj: Result<DetectRequest, serde_json::Error> = serde_json::from_str(req_body);
    let mut f = forest.lock().unwrap();
    let rsp_obj = match req_obj {
        Ok(req) => {
            let (op_status, matched_sieves) = f.detect(&req.filter_name, &req.content);
            serde_json::json!(
                {
                    "status": op_status.to_string(),
                    "time": start_time.elapsed().as_secs_f64(),
                    "hits": &matched_sieves,
                    "count": matched_sieves.len()
                }
            )
        },
        Err(_e) => serde_json::json!(
            {
                "status": ServiceStatus::RequestParameterError.to_string(),
                "time": start_time.elapsed().as_secs_f64()
            }
        )
    };
    HttpResponse::Ok()
    .content_type("application/json")
    .body(rsp_obj.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (host, port) = ("0.0.0.0", 80);
    config::init_config();
    let forest = Data::new(Mutex::new(filter::forest::FilterForest::new()));
    print_hello();
    println!("service running at {:}:{}", host, port);
    HttpServer::new(move || {
        App::new()
        .app_data(PayloadConfig::new(1000000 * 250))
        .app_data(forest.clone())
        .service(create)
        .service(add)
        .service(detect)
        .service(upload)
    })
    .bind((host, port))?
    .run()
    .await
}
