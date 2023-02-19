use actix_web::{HttpResponse};

pub async fn health_check() -> HttpResponse{
    println!("Health_Status: UP");
    HttpResponse::Ok().finish()
}
