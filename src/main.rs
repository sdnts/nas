use anyhow::*;
use std::path::Path;

mod app_state;
mod file;
mod routes;
mod templates;

#[async_std::main]
async fn main() -> Result<()> {
    tide::log::start();

    let state = app_state::AppState::new();
    let mut app = tide::with_state(state);

    let secret =
        dotenv::var("NAS_COOKIE_SECRET").context("[main] Unable to locate NAS_COOKIE_SECRET")?;
    app.with(tide::sessions::SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        secret.as_bytes(),
    ));

    app.at("/auth").get(routes::auth::get);
    app.at("/auth").post(routes::auth::post);

    app.at("/fs").get(routes::fs::get);
    app.at("/fs/*path").get(routes::fs::get);
    app.at("/fs/*path").post(routes::fs::post);
    app.at("/fs/*path").put(routes::fs::put);
    app.at("/fs/*path").delete(routes::fs::delete);

    app.at("/stream/*path").get(routes::stream::get);

    app.at("/public").serve_dir(Path::new("public/"))?;

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
