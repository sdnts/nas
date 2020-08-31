use std::path::Path;

mod error;
mod file;
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

    app.at("/fs").get(routes::fs::get);
    app.at("/fs/*path").get(routes::fs::get);
    app.at("/fs/*path").post(routes::fs::post);
    app.at("/fs/*path").delete(routes::fs::delete);

    app.at("/stream/*path").get(routes::stream::get);

    app.at("/public").serve_dir(Path::new("public/"))?;

    app.listen("0.0.0.0:8080").await?;

    Ok(())
}
