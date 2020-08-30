use anyhow::Result;
use std::fs;

use crate::error::NASError;
use crate::path::NASPath;

pub(crate) async fn get(req: tide::Request<()>) -> Result<String, tide::Error> {
    let relative_path: String = req.param("filename").unwrap_or_default();
    let path = NASPath::from_relative_path_str(&relative_path)?;
    let path = path.to_absolute_path_str()?;

    let contents =
        fs::read_dir(&path).map_err(|e| NASError::DirectoryNotFoundError(e.to_string()))?;
    let contents: Result<Vec<_>, _> = contents
        .map(move |f| -> Result<String> {
            let file = f.map_err(|e| NASError::UnknownError(e.to_string()))?;
            let file = NASPath::from_pathbuf(file.path());
            let file = file.to_relative_path_str()?;

            Ok(file.to_string())
        })
        .collect();

    let contents = contents.map_err(|e| NASError::UnknownError(e.to_string()))?;

    Ok(format!("{:?}", contents))
}

pub(crate) async fn post(_: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    unimplemented!()
}

pub(crate) async fn delete(_: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    unimplemented!()
}
