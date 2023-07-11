use actix_web::{ get, web::ServiceConfig, HttpResponse };

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", "0.0.1"))
        .finish()
}

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(health);
}