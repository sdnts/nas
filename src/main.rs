use actix_files::Files;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer, Result};
use lazy_static::lazy_static;

mod app_state;
mod auth;
mod config;
mod db;
mod error;
mod file;
mod fs;
mod hbs_helpers;
mod schema;
mod stream;
mod templates;
mod utils;

use app_state::AppState;
use error::NASError;

lazy_static! {
    // Unwrap all failables, because we want the panics

    static ref CONFIG: config::NASConfig = config::NASConfig {
        fs_root: "/home/ozark/nas_root".to_string(),
        cookie_secret: dotenv::var("NAS_COOKIE_SECRET")
            .unwrap(),
        theme: config::NASTheme::Dark
    };
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
            .service(
                web::scope("/auth")
                    .route("/", web::get().to(auth::get))
                    .route("/", web::post().to(auth::post)),
            )
            .service(
                web::scope("/fs")
                    .route("/{path:.*}", web::get().to(fs::get))
                    .route("/{path:.*}", web::post().to(fs::post))
                    .route("/{path:.*}", web::put().to(fs::put))
                    .route("/{path:.*}", web::delete().to(fs::delete)),
            )
            .service(web::scope("/stream").route("/{path:.*}", web::get().to(stream::get)))
            .service(Files::new("/public", "./public").show_files_listing())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())
}
