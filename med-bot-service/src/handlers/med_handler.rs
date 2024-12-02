use crate::dto::search_model::ApiSearchRequest;
use crate::AppState;
use actix_web::web::Json;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/med/search")]
async fn search_med(data: web::Data<AppState>, api_search_request: Json<ApiSearchRequest>) -> impl Responder {
    println!("search_med");

    let result = data.service.med_service
        .search_med(
            &data.client,
            api_search_request.search_key.to_owned(),
            api_search_request.city_id.to_owned(),
            api_search_request.subject_id.to_owned(),
        )
        .await;

    match result {
        Ok(response) => {
            // Serialize ApiResponse into JSON and return in HttpResponse
            match serde_json::to_string(&response) {
                Ok(json_response) => HttpResponse::Ok()
                    .content_type("application/json")
                    .body(json_response),
                Err(e) => {
                    eprintln!("{:?}", e);
                    HttpResponse::InternalServerError()
                        .body("Failed to serialize response to JSON")
                }
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Request failed: {}", e)),
    }
}

#[get("/med/appointments")]
async fn get_appointments(data: web::Data<AppState>) -> impl Responder {
    println!("get_appointments");

    let result = data.service.med_service
        .get_appointments(
            &data.client,
            "test_subject_id".parse().unwrap(),
            "test_doctor_id".parse().unwrap(),
            "test_service_id".parse().unwrap(),
            "test_partner_id".parse().unwrap(),
        )
        .await;

    match result {
        Ok(response) => {
            // Try to deserialize the response into ApiResponse
            match serde_json::to_string(&response) {
                Ok(json_response) => HttpResponse::Ok()
                    .content_type("application/json")
                    .body(json_response),
                Err(e) => {
                    eprintln!("{:?}", e);
                    HttpResponse::InternalServerError()
                        .body("Failed to serialize response to JSON")
                }
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Request failed: {}", e)),
    }
}

#[get("/med/appointments/analyze")]
async fn analyze(data: web::Data<AppState>) -> impl Responder {
    println!("analyze_med");

    let result = data.service.med_service.analyze_appointment(&data.client).await;
    match result {
        Ok(response) => {
            // Try to deserialize the response into ApiResponse
            match serde_json::to_string(&response) {
                Ok(json_response) => HttpResponse::Ok()
                    .content_type("application/json")
                    .body(json_response),
                Err(e) => {
                    eprintln!("{:?}", e);
                    HttpResponse::InternalServerError()
                        .body("Failed to serialize response to JSON")
                }
            }
        }
        Err(e) => HttpResponse::NotFound()
            .body(format!("Request unavailable: {}", e)),
    }
}

#[get("/med/doctor")]
async fn get_doctor(data: web::Data<AppState>) -> impl Responder {
    println!("get_doctor");

    let result = data.service.med_service.get_doctor().await;
    match result {
        Ok(response) => {
            // Try to deserialize the response into ApiResponse
            match serde_json::to_string(&response) {
                Ok(json_response) => HttpResponse::Ok()
                    .content_type("application/json")
                    .body(json_response),
                Err(e) => {
                    eprintln!("{:?}", e);
                    HttpResponse::InternalServerError()
                        .body("Failed to serialize response to JSON")
                }
            }
        }
        Err(e) => HttpResponse::NotFound()
            .body(format!("Request unavailable: {}", e)),
    }
}

