use actix_web::{http, HttpResponse, Responder};

pub async fn get() -> impl Responder {
    HttpResponse::PermanentRedirect()
        .header(http::header::LOCATION, "/fs/")
        .finish()
}
