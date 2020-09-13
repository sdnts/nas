use actix_web::Responder;

pub async fn post() -> impl Responder {
    format!("/fs post")
}
