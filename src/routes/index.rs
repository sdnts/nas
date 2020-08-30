use anyhow::Result;
use askama::Template;
use std::fs;

use crate::error::NASError;
use crate::path::NASPath;

#[derive(Template)]
#[template(path = "index.html")]
struct T {
    name: String,
    file_list: Vec<String>,
}

pub(crate) async fn get(req: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    let relative_path: String = req.param("path").unwrap_or_default();
    let path = NASPath::from_relative_path_str(&relative_path)?;
    let path = path.to_absolute_path_str()?;

    let contents =
        fs::read_dir(&path).map_err(|e| NASError::DirectoryNotFoundError(e.to_string()))?;
    let file_list: Result<Vec<_>, _> = contents
        .map(move |f| -> Result<String> {
            let file = f.map_err(|e| NASError::UnknownError(e.to_string()))?;
            let file = NASPath::from_pathbuf(file.path());
            let file = file.to_relative_path_str()?;

            Ok(file.to_string())
        })
        .collect();
    let file_list = file_list.map_err(|e| NASError::UnknownError(e.to_string()))?;

    let t = T {
        name: "0zark".to_string(),
        file_list,
    };
    let res = t.render().unwrap();

    Ok(tide::Response::builder(200)
        .body(res)
        .content_type("text/html;charset=utf-8")
        .build())
}
