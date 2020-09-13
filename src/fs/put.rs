use actix_web::Responder;

pub async fn put() -> impl Responder {
    format!("/fs put")
}
