use actix_web::Responder;

pub async fn post() -> impl Responder {
    format!("/auth post")
}
