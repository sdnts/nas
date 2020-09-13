use actix_web::{web, HttpResponse, Responder, Result};
use flate2::{write::GzEncoder, Compression};
use std::fs;
use std::io::prelude::*;

use crate::error::NASError;
use crate::file::{NASFile, NASFileCategory};
use crate::utils::strip_trailing_char;

pub async fn get(path: web::Path<String>) -> Result<impl Responder> {
    // The NormalizePath middleware will add a trailing slash at the end of the path, so we must remove it
    let path = strip_trailing_char(path.clone());
    let nas_file = NASFile::from_relative_path_str(&path)?;

    let response = {
        let mut file = fs::File::open(&nas_file).map_err(|_| NASError::PathReadError {
            path: nas_file.relative_path_str.to_string(),
        })?;
        let mut file_bytes = Vec::new();
        file.read_to_end(&mut file_bytes)
            .map_err(|_| NASError::PathReadError {
                path: nas_file.relative_path_str.to_string(),
            })?;

        // Gzip it
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&file_bytes)?;
        let file_bytes = encoder.finish()?;

        // And send it back
        HttpResponse::Ok()
            .body(file_bytes)
            .with_header("Access-Control-Allow-Origin", "*")
            .with_header("Content-Encoding", "gzip")
            .with_header("Access-Control-Expose-Headers", "Content-Length")
            .with_header("Access-Control-Allow-Headers", "Range")
            .with_header("Content-Type", {
                match nas_file.category {
                    NASFileCategory::StreamPlaylist => "application/vnd.apple.mpegurl",
                    NASFileCategory::StreamSegment => "application/octet-stream",
                    _ => "",
                }
            })
    };

    Ok(response)
}
