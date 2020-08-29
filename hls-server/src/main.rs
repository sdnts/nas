use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::prelude::*;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();

    let mut app = tide::new();

    app.at("/").get(|_| async {
        Ok(tide::Response::builder(200)
            .body(
                "
        <html>
            <head>
                <meta charset=\"UTF-8\">
            </head>
            <body>
                <script src=\"//cdn.jsdelivr.net/npm/hls.js@latest\"></script>
                <video id=\"video\" width=\"1280\" height=\"720\" controls></video>
                <script>
                    if (Hls.isSupported()) {
                        var video = document.getElementById('video');
                        var hls = new Hls({ debug: true });
                        hls.loadSource(`https://0b4ec16ca43a.ngrok.io/bbb.m3u8`);
                        // hls.loadSource(`https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8`)
                        hls.attachMedia(video);
                        hls.on(Hls.Events.MEDIA_ATTACHED, function() {
                            video.play();
                        });
                        hls.on(Hls.Events.ERROR, function (event, data) {
                            console.log(`Error in stream`, event, data)
                        });
                    } else if (video.canPlayType('application/vnd.apple.mpegurl')) {
                        video.src = 'https://test-streams.mux.dev/x36xhzz/x36xhzz.m3u8';
                        video.addEventListener('canplay',function() {
                            video.play();
                        });
                    }
                </script>
            </body>
        </html>
        ",
            )
            .header("Content-Type", "text/html")
            .build())
    });

    app.at("/bbb.m3u8").get(|_| async move {
        let path = std::path::Path::new("/home/ozark/Video/hls/index.m3u8");

        let mut file = std::fs::File::open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();

        let response = tide::Response::builder(200)
            .body(bytes)
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Expose-Headers", "Content-Length")
            .header("Access-Control-Allow-Headers", "Range")
            .header("Content-Type", "application/vnd.apple.mpegurl");
        Ok(response.build())
    });

    app.at("/:file").get(|req: tide::Request<()>| async move {
        let filename: String = req.param("file")?;
        let path = std::path::Path::new("/home/ozark/Video/hls").join(&filename);

        let mut file = std::fs::File::open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&bytes)?;
        let bytes = encoder.finish()?;

        let response = tide::Response::builder(200)
            .body(bytes)
            .header("Access-Control-Allow-Origin", "*")
            .header("Content-Encoding", "gzip")
            .header("Access-Control-Expose-Headers", "Content-Length")
            .header("Access-Control-Allow-Headers", "Range")
            .header("Content-Type", "application/octet-stream");

        println!("Responding");
        Ok(response.build())
    });

    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
