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

    let contents = fs::read_dir(&path).map_err(|e| e)?;
    let contents = contents
        .map(move |f| {
            let file = f.unwrap();
            let file = file.path();
            let file = file
                .strip_prefix(&ROOT)
                // .map_err(|_| NASError::UnknownError("Could not strip FS prefix".to_string()))?;
                .unwrap();
            file.to_str().unwrap().to_string()
        })
        .collect::<Vec<String>>();

    Ok(format!("{:?}", contents))
}
