mod models;
mod scheduler;
mod handlers;
mod repositories;
mod config;
mod services;
mod dto;

use std::env;
use crate::config::mongo_config::{MongoClient, MongoClientBuilder};
use crate::handlers::med_handler;
use crate::scheduler::start_scheduler;
use crate::services::med_service;
use crate::services::med_service::{MedService, MedServiceBuilder};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use reqwest::Client;
use crate::config::mail_config::MailClientBuilder;
use crate::config::med_target_config::MedTargetBuilder;
use crate::services::mail_service::MailServiceBuilder;

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
    // Load the correct .env file based on the environment
    let env_profile = env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());

    match env_profile.as_str() {
        "local" => dotenv::from_filename(".env.local").ok(),
        "dev" => dotenv::from_filename("../../.env.dev").ok(),
        "release" => dotenv::from_filename("../../.env.release").ok(),
        _ => dotenv().ok(),  // Default to loading .env
    };

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting HTTP server: go to http://0.0.0.0:8082");

    // Config
    let mongo_client = MongoClientBuilder::new().await
        .with_dynamic_collection()
        .with_user_collection()
        .with_doctor_collection()
        .build();

    let mail_client = MailClientBuilder::new()
        .build();

    let med_target = MedTargetBuilder::new()
        .build();

    // Service
    let mail_service = MailServiceBuilder::new(mail_client)
        .build();

    let med_service = MedServiceBuilder::new(
        med_target,
        mongo_client.doctor_collection.clone(),
        mail_service.clone(),
    )
        .build();

    let app_state = web::Data::new(AppState {
        client: Client::new(),
        mongo_client,
        service: ServiceState {
            med_service,
        },
    });


    // Start scheduler on a new thread
    let scheduler_state = app_state.clone();
    actix_rt::spawn(async move {
        start_scheduler(scheduler_state).await;
    });

    HttpServer::new(move || {
        let state_clone = app_state.clone();
        App::new()
            .app_data(state_clone)
            .service(health)
            .service(get_ips)
            .service(med_handler::search_med)
            .service(med_handler::get_appointments)
            .service(med_handler::analyze)
            .service(med_handler::get_doctor)
    })
        .bind(("0.0.0.0", 8082))?
        .run()
        .await
}
