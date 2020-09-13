use actix_web::Responder;

pub async fn get() -> impl Responder {
    format!("/auth get")
}
