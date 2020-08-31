use std::io::prelude::*;

use anyhow::Result;
use flate2::write::GzEncoder;
use flate2::Compression;

use crate::error::NASError;
use crate::path::NASPath;

pub(crate) async fn get(mut req: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    let path: String = req.param("path")?;

    let nas_path = NASPath::from_relative_path_str(&path)?;
    let extension = nas_path
        .to_pathbuf()
        .extension()
        .ok_or(NASError::InvalidExtensionError(path.clone()))?;
    let extension = extension
        .to_str()
        .ok_or(NASError::InvalidExtensionError(path.clone()))?;

    let response = match extension {
        "m3u8" => {
            // This request is asking for a playlist

            // Attach the playlist path to the session
            let session = req.session_mut();
            session.insert("stream_playlist", path.clone())?;

            // Open the playlist file
            let playlist_path = nas_path.to_absolute_path_str()?;
            let mut file = std::fs::File::open(playlist_path)?;
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
        }
        "ts" => {
            // This request is asking for a video segment

            // Find the playlist for this segment (from the session)
            let playlist: String = req
                .session()
                .get("stream_playlist")
                .ok_or(NASError::MissingStreamPlaylist)?;

            // Get the playlist path
            let playlist_path = NASPath::from_relative_path_str(&playlist)?;
            let playlist_path = playlist_path.to_pathbuf();
            // Use the playlist path to figure out actual path for this segment
            let segment_dir = playlist_path
                .parent()
                .ok_or(NASError::InvalidStreamPlaylist(playlist))?;
            let segment_path = segment_dir.join(&path);

            // Open the segment file
            let mut segment = std::fs::File::open(segment_path)?;
            let mut segment_bytes = Vec::new();
            segment.read_to_end(&mut segment_bytes)?;

            // Gzip it
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&segment_bytes)?;
            let segment_bytes = encoder.finish()?;

            // And send it back
            tide::Response::builder(200)
                .body(segment_bytes)
                .header("Access-Control-Allow-Origin", "*")
                .header("Content-Encoding", "gzip")
                .header("Access-Control-Expose-Headers", "Content-Length")
                .header("Access-Control-Allow-Headers", "Range")
                .header("Content-Type", "application/octet-stream")
                .build()
        }
        _ => tide::Response::builder(500).build(),
    };

    Ok(response)
}
