use std::io::prelude::*;

use anyhow::Result;
use flate2::write::GzEncoder;
use flate2::Compression;

use crate::error::NASError;
use crate::path::NASPath;

pub(crate) async fn get(mut req: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    let filename: String = req.param("file")?;

    let response = {
        if filename.ends_with(".m3u8") {
            let session = req.session_mut();

            let path = NASPath::from_relative_path_str(&filename)?;
            let path = path.to_absolute_path_str()?;
            println!("Path 0 {:?}", path);

            let path = NASPath::from_relative_path_str(&filename)?;
            let path = path.to_absolute_path_str()?;
            session.insert("stream_playlist", path)?;

            let mut file = std::fs::File::open(path)?;
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).unwrap();

            let response = tide::Response::builder(200)
                .body(bytes)
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Headers", "Range")
                .header("Access-Control-Expose-Headers", "Content-Length")
                .header("Content-Type", "application/vnd.apple.mpegurl");

            response.build()
        } else if filename.ends_with(".ts") {
            let playlist: String = req
                .session()
                .get("stream_playlist")
                .ok_or(NASError::MissingStreamPlaylist)?;

            println!("Playlist found {}", playlist);

            let path = NASPath::from_relative_path_str(&playlist)?;
            let path = path.to_pathbuf();
            println!("Path 1 {:?}", path);
            let path = path
                .parent()
                .ok_or(NASError::InvalidStreamPlaylist(playlist))?;
            println!("Path.parent found {:?}", path);
            let path = path.join(&filename);

            println!("Segment found {:?}", path);

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

            response.build()
        } else {
            tide::Response::builder(500).build()
        }
    };

    Ok(response)
}
