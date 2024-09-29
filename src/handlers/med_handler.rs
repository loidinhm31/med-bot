use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};

#[get("/med/search")]
async fn search_med(data: web::Data<AppState>) -> impl Responder {
    println!("search_med");

    let result = data.service.med_service
        .search_med(&data.client)
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
        .get_appointments(&data.client, "umc_P2", "umc_453", "umc_service-150000")
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

#[get("/med/analyze")]
async fn analyze(data: web::Data<AppState>) -> impl Responder {
    println!("analyze_med");

    let result = data.service.med_service.check_appointment(&data.client).await;
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

