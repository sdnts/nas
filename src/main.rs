use askama::Template;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::prelude::*;
use std::path::Path;

mod error;
mod path;
mod routes;

#[derive(Template)]
#[template(path = "index.html")]
struct T {
    name: String,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();

    let mut app = tide::new();

    app.with(tide::sessions::SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        dotenv::var("NAS_COOKIE_SECRET").unwrap().as_bytes(),
    ));

    app.at("/").serve_dir(Path::new("public/"))?;
    app.at("/").get(|_| async {
        let t = T {
            name: "0zark".to_string(),
        };
        let res = t.render().unwrap();

        Ok(tide::Response::builder(200).body(res).build())
    });

    app.at("/api/fs/*file").get(routes::fs::get);
    app.at("/api/fs/*file").post(routes::fs::post);
    app.at("/api/fs/*file").delete(routes::fs::delete);

    app.at("/api/stream/*file").get(routes::stream::get);

    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
