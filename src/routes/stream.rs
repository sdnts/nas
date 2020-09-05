use std::io::prelude::*;

use anyhow::Result;
use flate2::write::GzEncoder;
use flate2::Compression;

use crate::file::{NASFile, NASFileType};

pub(crate) async fn get(mut req: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    let path: String = req.param("path")?;

    let nas_file = NASFile::from_relative_path_str(&path)?;
    let response = match nas_file.file_type {
        NASFileType::StreamPlaylist => {
            // This request is asking for a playlist

            // Attach the playlist path to the session
            let session = req.session_mut();
            session.insert("stream_playlist", path.clone())?;

            // Open the playlist file
            let playlist_path = nas_file.to_absolute_path_str()?;
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
        NASFileType::StreamSegment => {
            // This request is asking for a video segment

            // Open the segment file
            let mut segment = std::fs::File::open(nas_file.path)?;
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
