use std::io::Read;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();

    let mut app = tide::new();

    app.at("/").get(|_| async {
        Ok(tide::Response::builder(200)
            .body(
                "
        <html>
            <body>
                <video width=\"640\" height:\"480\" controls src=\"./index.m3u8\">
                </video>
            </body>
        </html>
        ",
            )
            .header("Content-Type", "text/html")
            .build())
    });

    app.at("/:file").get(|req: tide::Request<()>| async move {
        let filename: String = req.param("file").unwrap_or("index.m3u8".to_string());
        let path =
            std::path::Path::new("/home/ozark/Video/The Big Lebowski (1998)/hls").join(&filename);

        let mut file = std::fs::File::open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        let parsed = m3u8_rs::parse_playlist_res(&bytes);

        match parsed {
            Ok(m3u8_rs::playlist::Playlist::MasterPlaylist(_)) => unimplemented!(),
            Ok(m3u8_rs::playlist::Playlist::MediaPlaylist(p)) => {
                println!("{:?}", p.version);
                // p.segments.iter().for
            }
            Err(e) => println!("Error: {:?}", e),
        }

        let response = tide::Response::builder(200)
            .body(bytes)
            .header("Content-Type", {
                if filename.ends_with(".m3u8") {
                    "vnd.apple.mpegURL"
                } else if filename.ends_with(".ts") {
                    "video/MP2T"
                } else {
                    "video/mp4"
                }
            });
        Ok(response.build())
    });

    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
