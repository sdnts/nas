use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};
use anyhow::*;
use lazy_static::lazy_static;

mod app_state;
mod auth;
mod config;
mod error;
mod file;
mod fs;
mod hbs_helpers;
mod templates;
mod utils;
// mod db;
// mod routes;
// mod schema;
// mod session;
use app_state::AppState;

lazy_static! {
    // Unwrap all failables, because we want the panics

    static ref CONFIG: config::NASConfig = config::NASConfig {
        fs_root: "/home/ozark/nas_root/0zark".to_string(),
        cookie_secret: dotenv::var("NAS_COOKIE_SECRET")
            .context("[main] Unable to locate NAS_COOKIE_SECRET")
            .unwrap(),
        hostname: "0zark".to_string(),
        theme: config::NASTheme::Dark
    };
}

#[actix_rt::main]
async fn main() -> Result<()> {
    let app_state = web::Data::new(AppState::new()?);

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(Files::new("/public", "./public").show_files_listing())
            .wrap(middleware::NormalizePath::default())
            .route("/auth", web::get().to(auth::get))
            .route("/auth", web::post().to(auth::post))
            .route("/fs/{path:.*}", web::get().to(fs::get))
            .route("/fs/{path:.*}", web::post().to(fs::post))
            .route("/fs/{path:.*}", web::put().to(fs::put))
            .route("/fs/{path:.*}", web::delete().to(fs::delete))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await?;

    Ok(())

    // tide::log::start();
    // db::NASDB::init()?;

    // let state = app_state::AppState::new()?;
    // let mut app = tide::with_state(state);

    // app.with(tide::sessions::SessionMiddleware::new(
    //     tide::sessions::CookieStore::new(),
    //     // session::NASSessionStore::new(),
    //     CONFIG.cookie_secret.as_bytes(),
    // ));

    // // Unprotected paths
    // app.at("/auth").get(routes::auth::get);
    // app.at("/auth").post(routes::auth::post);

    // app.at("/public").serve_dir(Path::new("public/"))?;

    // // Protected paths
    // app.at("/fs").get(routes::fs::get);
    // app.at("/fs/*path").get(routes::fs::get);
    // app.at("/fs/*path").post(routes::fs::post);
    // app.at("/fs/*path").put(routes::fs::put);
    // app.at("/fs/*path").delete(routes::fs::delete);

    // app.at("/stream/*path").get(routes::stream::get);

    // app.listen("0.0.0.0:8080").await?;

    // Ok(())
}
