use actix_web::HttpResponse;
// jsut an health_check route to check if the server is live and taking request
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
