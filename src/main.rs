mod models;
mod scheduler;
mod handlers;
mod repositories;
mod config;
mod services;

use crate::config::config::{MongoClient, MongoClientBuilder};
use crate::handlers::med_handler;
use crate::scheduler::start_scheduler;
use crate::services::med_service::{MedService, MedServiceBuilder};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;


#[get("/healthz")]
async fn health() -> impl Responder {
    "Up".to_string()
}

#[get("/ipz")]
async fn get_ips() -> impl Responder {
    let result = reqwest::get("https://httpbin.org/ip")
        .await;
    match result {
        Ok(response) => {
            HttpResponse::Ok().content_type("application/json").body(response.text().await.unwrap())
        }
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

struct AppState {
    client: Client,
    mongo_client: MongoClient,
    service: ServiceState,
}

struct ServiceState {
    med_service: MedService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting HTTP server: go to http://127.0.0.1:8082");

    let mongo_client = MongoClientBuilder::new().await
        .with_user_collection()
        .with_dynamic_collection()
        .build();

    let med_service = MedServiceBuilder::new().build();

    let app_state = web::Data::new(AppState {
        client: Client::new(),
        mongo_client,
        service: ServiceState {
            med_service,
        },
    });

    // Start scheduler on a new thread
    actix_rt::spawn(async move {
        start_scheduler().await;
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(health)
            .service(get_ips)
            .service(med_handler::search_med)
            .service(med_handler::get_appointments)
            .service(med_handler::analyze)
    })
        .bind(("127.0.0.1", 8082))?
        .run()
        .await
}
