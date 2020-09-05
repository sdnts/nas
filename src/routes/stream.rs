use std::io::prelude::*;

use anyhow::Result;
use flate2::write::GzEncoder;
use flate2::Compression;

use crate::file::{NASFile, NASFileType};

pub(crate) async fn get(req: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    let path: String = req.param("path")?;

    let nas_file = NASFile::from_relative_path_str(&path)?;
    let response = {
        let mut file = std::fs::File::open(nas_file.path)?;
        let mut file_bytes = Vec::new();
        file.read_to_end(&mut file_bytes)?;

        // Gzip it
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&file_bytes)?;
        let file_bytes = encoder.finish()?;

        // And send it back
        tide::Response::builder(200)
            .body(file_bytes)
            .header("Access-Control-Allow-Origin", "*")
            .header("Content-Encoding", "gzip")
            .header("Access-Control-Expose-Headers", "Content-Length")
            .header("Access-Control-Allow-Headers", "Range")
            .header("Content-Type", {
                match nas_file.file_type {
                    NASFileType::StreamPlaylist => "application/vnd.apple.mpegurl",
                    NASFileType::StreamSegment => "application/octet-stream",
                    _ => "",
                }
            })
            .build()
    };

    Ok(response)
}
