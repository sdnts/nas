use anyhow::*;
use lazy_static::lazy_static;
use std::path::Path;

mod app_state;
mod config;
mod db;
mod file;
mod hbs_helpers;
mod routes;
mod schema;
mod session;
mod templates;

lazy_static! {
    // Unwrap all failables, because we want the panics

    static ref CONFIG: config::NASConfig = config::NASConfig {
        fs_root: "/home/ozark/nas_root/".to_string(),
        cookie_secret: dotenv::var("NAS_COOKIE_SECRET")
            .context("[main] Unable to locate NAS_COOKIE_SECRET")
            .unwrap(),
        hostname: "0zark".to_string(),
        theme: config::NASTheme::Dark
    };
}

#[async_std::main]
async fn main() -> Result<()> {
    tide::log::start();
    db::NASDB::init()?;

    let state = app_state::AppState::new()?;
    let mut app = tide::with_state(state);

    app.with(tide::sessions::SessionMiddleware::new(
        session::NASSessionStore::new(),
        CONFIG.cookie_secret.as_bytes(),
    ));

    // Unprotected paths
    app.at("/auth").get(routes::auth::get);
    app.at("/auth").post(routes::auth::post);

    app.at("/public").serve_dir(Path::new("public/"))?;

    // Protected paths
    app.at("/fs").get(routes::fs::get);
    app.at("/fs/*path").get(routes::fs::get);
    app.at("/fs/*path").post(routes::fs::post);
    app.at("/fs/*path").put(routes::fs::put);
    app.at("/fs/*path").delete(routes::fs::delete);

    app.at("/stream/*path").get(routes::stream::get);

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
