use actix_web::Responder;

pub async fn delete() -> impl Responder {
    format!("/fs delete")
}
