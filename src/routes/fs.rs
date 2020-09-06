use anyhow::*;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

use crate::app_state::AppState;
use crate::file::{NASFile, NASFileCategory};
use crate::templates::{BadRequestPageParams, FileListPageParams, StreamPageParams};

pub async fn get(req: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    let templates = req.state().clone().templates;
    let path: String = req.param("path").unwrap_or_default();

    let nas_file = NASFile::from_relative_path_str(&path)?;

    let response_body = {
        match nas_file.category {
            NASFileCategory::Directory => {
                // For directories, render the file list page
                let contents = fs::read_dir(&nas_file)
                    .with_context(|| format!("[fs::get] Unable to read_dir: {:?}", nas_file))?;
                let files = contents
                    .map(move |f| -> Result<NASFile> {
                        let file = f.context("[fs::get] Failed to get DirEntry ")?;
                        let file = NASFile::from_pathbuf(file.path())?;
                        Ok(file)
                    })
                    .collect::<Result<Vec<NASFile>>>()?;

                let breadcrumbs: PathBuf = nas_file.into();
                let breadcrumbs = breadcrumbs
                    .iter()
                    .map(|component| -> Result<_> {
                        let component = component.to_str().with_context(|| {
                            format!(
                                "[fs::get] Failed to convert &OsStr to &str: {:?}",
                                component
                            )
                        })?;
                        Ok(component.to_string())
                    })
                    .collect::<Result<Vec<String>>>()?;

                let parent_href: Vec<String> = breadcrumbs
                    .iter()
                    .take(breadcrumbs.len() - 1)
                    .map(|b| b.to_string())
                    .collect();
                let parent_href = parent_href.join("/");

                templates.render(
                    "fs",
                    &FileListPageParams {
                        title: "/fs".to_string(),
                        hostname: "0zark".to_string(),
                        username: "0zark".to_string(),
                        breadcrumbs,
                        parent_href,
                        files,
                    },
                )?
            }
            NASFileCategory::StreamPlaylist => templates.render(
                "stream",
                &StreamPageParams {
                    hostname: "0zark".to_string(),
                    src: format!("/stream/{}", path),
                    file_name: "S01E02".to_string(),
                },
            )?,
            _ => templates.render(
                "400",
                &BadRequestPageParams {
                    title: "/fs".to_string(),
                    hostname: "0zark".to_string(),
                    username: "0zark".to_string(),
                },
            )?,
        }
    };

    let response = tide::Response::builder(200)
        .body(response_body)
        .content_type("text/html;charset=utf-8")
        .build();

    Ok(response)
}

#[derive(Debug, Deserialize)]
struct FSPUTParams {
    pub dir_name: String,
}
pub async fn put(mut req: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    // let templates = req.state().clone().templates;
    // let path: String = req.param("path").unwrap_or_default();
    // let body: FSPUTParams = req.body_form().await?;

    // let nas_file = NASFile::from_relative_path_str(&path)?;
    // let pathbuf = PathBuf::new().join(nas_file.absolute_path_str);

    // if let Some(dir_name) = body.dir_name {
    //     // Create new dir at this path
    //     let pathbuf = pathbuf.join(dir_name);
    //     fs::create_dir_all(pathbuf)?;
    // } else if let Some(name) = body.name {
    //     // Rename this path
    //     dbg!(&pathbuf);
    //     let renamed_pathbuf = pathbuf.join(name);
    //     dbg!(&renamed_pathbuf);
    //     fs::rename(pathbuf, renamed_pathbuf)?;
    // } else {
    //     // Bad request
    // }

    Ok(tide::Response::builder(200).build())
}

pub async fn post(mut req: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    let templates = req.state().clone().templates;
    let path: String = req.param("path").unwrap_or_default();

    dbg!(&path);
    let nas_file = NASFile::from_relative_path_str(&path)?;
    dbg!(&nas_file);

    let body: Result<FSPUTParams, tide::Error> = req.body_form().await;

    if let Ok(body) = body {
        // Create Dir
        println!("dsd");
        let dir_name = body.dir_name;
        let pathbuf = PathBuf::new().join(nas_file.absolute_path_str);
        let pathbuf = pathbuf.join(dir_name);
        fs::create_dir_all(pathbuf)?;
    } else {
        // Create file
        use async_std::{fs::OpenOptions, io};
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&nas_file.absolute_path_str)
            .await?;

        io::copy(req, file).await?;
    }

    Ok(tide::Response::builder(200).build())
}

pub async fn delete(_: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    unimplemented!()
}
