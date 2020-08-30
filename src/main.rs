use std::path::Path;

mod error;
mod path;
mod routes;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();

    let mut app = tide::new();

    app.with(tide::sessions::SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        dotenv::var("NAS_COOKIE_SECRET").unwrap().as_bytes(),
    ));

    app.at("/api/fs/").get(routes::fs::get);
    app.at("/api/fs/*filename").get(routes::fs::get);
    app.at("/api/fs/*filename").post(routes::fs::post);
    app.at("/api/fs/*filename").delete(routes::fs::delete);

    app.at("/api/stream/*filename").get(routes::stream::get);

    app.at("/").get(routes::index::get);
    app.at("/*path").get(routes::index::get);

    app.at("/public").serve_dir(Path::new("public/"))?;

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
