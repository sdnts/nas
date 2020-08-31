use anyhow::Result;
use askama::Template;
use std::fs;

use crate::error::NASError;
use crate::path::NASPath;

#[derive(Template, Debug)]
#[template(path = "index.jinja")]
struct FileListPage {
    name: String,
    file_list: Vec<String>,
}

#[derive(Template, Debug)]
#[template(path = "stream.jinja", escape = "none")]
struct StreamPage {
    src: String,
}

#[derive(Template, Debug)]
#[template(path = "400.jinja")]
struct BadRequestPage {}

pub(crate) async fn get(req: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    let path: String = req.param("path").unwrap_or_default();

    let nas_path = NASPath::from_relative_path_str(&path)?;
    let absolute_path = nas_path.to_absolute_path_str()?;
    let nas_pathbuf = nas_path.to_pathbuf();

    let page = {
        if nas_pathbuf.is_dir() {
            // For directories, render the file list page

            let contents = fs::read_dir(&absolute_path)
                .map_err(|e| NASError::DirectoryNotFoundError(e.to_string()))?;
            let file_list: Result<Vec<_>, _> = contents
                .map(move |f| -> Result<String> {
                    let file = f.map_err(|e| NASError::UnknownError(e.to_string()))?;
                    let file = NASPath::from_pathbuf(file.path());
                    let file = file.to_relative_path_str()?;
                    Ok(file.to_string())
                })
                .collect();
            let file_list = file_list.map_err(|e| NASError::UnknownError(e.to_string()))?;

            let page = FileListPage {
                name: "0zark".to_string(),
                file_list,
            };
            page.render()?
        } else {
            // For files

            let extension = nas_path
                .to_pathbuf()
                .extension()
                .ok_or(NASError::InvalidExtensionError(path.clone()))?;
            let extension = extension
                .to_str()
                .ok_or(NASError::InvalidExtensionError(path.clone()))?;

            match extension {
                "m3u8" => {
                    // For stream playlist files, render the stream page
                    let page = StreamPage {
                        src: format!("/stream/{}", path),
                    };
                    page.render()?
                }
                _ => {
                    // For everything else, render the 400 page
                    let page = BadRequestPage {};
                    page.render()?
                }
            }
        }
    };

    let response = tide::Response::builder(200)
        .body(page)
        .content_type("text/html;charset=utf-8")
        .build();

    Ok(response)
}

pub(crate) async fn post(_: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    unimplemented!()
}

pub(crate) async fn delete(_: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    unimplemented!()
}
