use actix_files::Files;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer, Result};
use lazy_static::lazy_static;

mod app_state;
mod config;
mod db;
mod error;
mod file;
mod hbs_helpers;
mod routes;
mod templates;
mod utils;

use app_state::AppState;
use error::NASError;

lazy_static! {
    static ref CONFIG: config::NASConfig = Default::default();
}

#[actix_rt::main]
async fn main() -> Result<()> {
    db::NASDB::init().map_err(|_| NASError::DBInitializationError)?;

    let app_state = AppState::new()?;
    let app_state = web::Data::new(app_state);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::NormalizePath::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&CONFIG.cookie_secret.as_bytes())
                    .name("nas_session")
                    .path("/")
                    .secure(false),
            ))
            .route("/", web::get().to(routes::index::get))
            .service(
                web::scope("/auth")
                    .route("/", web::get().to(routes::auth::get))
                    .route("/", web::post().to(routes::auth::post))
                    .route("/", web::delete().to(routes::auth::delete)),
            )
            .service(
                web::scope("/fs")
                    .route("/{path:.*}", web::get().to(routes::fs::get))
                    .route("/{path:.*}", web::post().to(routes::fs::post))
                    .route("/{path:.*}", web::put().to(routes::fs::put))
                    .route("/{path:.*}", web::delete().to(routes::fs::delete)),
            )
            .service(web::scope("/stream").route("/{path:.*}", web::get().to(routes::stream::get)))
            .service(Files::new("/public", "./public").show_files_listing())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}
