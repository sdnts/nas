use std::io::prelude::*;

use anyhow::Result;
use flate2::write::GzEncoder;
use flate2::Compression;

use crate::error::NASError;
use crate::path::NASPath;

pub(crate) async fn get(mut req: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    let filename: String = req.param("filename")?;

    let response = {
        if filename.ends_with(".m3u8") {
            // This request is asking for a playlist

            // Get the playlist path
            let path = NASPath::from_relative_path_str(&filename)?;
            let path = path.to_absolute_path_str()?;

            // Attach the playlist path to the session
            let session = req.session_mut();
            session.insert("stream_playlist", path)?;

            // Open the playlist file
            let mut file = std::fs::File::open(path)?;
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes).unwrap();

            // And send it back
            tide::Response::builder(200)
                .body(bytes)
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Headers", "Range")
                .header("Access-Control-Expose-Headers", "Content-Length")
                .header("Content-Type", "application/vnd.apple.mpegurl")
                .build()
        } else if filename.ends_with(".ts") {
            // This request is asking for a video segment

            // Find the playlist for this segment (from the session)
            let playlist: String = req
                .session()
                .get("stream_playlist")
                .ok_or(NASError::MissingStreamPlaylist)?;

            // Get the playlist path
            let path = NASPath::from_relative_path_str(&playlist)?;
            let path = path.to_pathbuf();
            // Use the playlist path to figure out actual path for this segment
            let path = path
                .parent()
                .ok_or(NASError::InvalidStreamPlaylist(playlist))?;
            let path = path.join(&filename);

            // Open the segment file
            let mut file = std::fs::File::open(path)?;
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;

            // Gzip it
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&bytes)?;
            let bytes = encoder.finish()?;

            // And send it back
            tide::Response::builder(200)
                .body(bytes)
                .header("Access-Control-Allow-Origin", "*")
                .header("Content-Encoding", "gzip")
                .header("Access-Control-Expose-Headers", "Content-Length")
                .header("Access-Control-Allow-Headers", "Range")
                .header("Content-Type", "application/octet-stream")
                .build()
        } else {
            // This request is unrecognized
            tide::Response::builder(500).build()
        }
    };

    Ok(response)
}
