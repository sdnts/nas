use std::fs;
use std::path::Path;

use crate::error::NASError;

const ROOT: &str = "/home/ozark/nas";

pub(crate) async fn get(req: tide::Request<()>) -> Result<String, tide::Error> {
    let relative_path = req.param::<String>("file").unwrap_or(String::from("/"));
    let relative_path = percent_encoding::percent_decode_str(&relative_path).decode_utf8()?;

    let path = Path::new(ROOT).join(&*relative_path);
    let path = path
        .to_str()
        .ok_or(NASError::InvalidPathError(relative_path.to_string()))?;

    let contents =
        fs::read_dir(&path).map_err(|e| NASError::DirectoryNotFoundError(e.to_string()))?;
    let contents: Result<Vec<_>, _> = contents
        .map(move |f| -> Result<String, NASError> {
            let file = f.map_err(|e| NASError::UnknownError(e.to_string()))?;
            let file = file.path();
            let file = file
                .strip_prefix(&ROOT)
                .map_err(|_| NASError::UnknownError("Could not strip FS prefix".to_string()))?;
            let file = file
                .to_str()
                .ok_or(NASError::InvalidPathError("".to_string()))?;

            Ok(file.to_string())
        })
        .collect();

    let contents = contents.map_err(|e| NASError::UnknownError(e.to_string()));

    Ok(format!("{:?}", contents))
}
