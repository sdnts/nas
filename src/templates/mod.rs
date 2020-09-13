use serde::Serialize;

use crate::file::NASFile;

#[derive(Serialize)]
pub struct BadRequestPageParams {
    pub title: String,
    pub hostname: String,
    pub username: String,
}

#[derive(Serialize)]
pub struct UnauthorizedPageParams {
    pub title: String,
    pub hostname: String,
    pub username: String,
}

#[derive(Serialize)]
pub struct AuthPageParams {
    pub title: String,
    pub hostname: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct FSPageParams {
    pub title: String,
    pub hostname: String,
    pub username: String,
    pub breadcrumbs: Vec<String>,
    pub parent_href: String,
    pub files: Vec<NASFile>,
}

#[derive(Serialize)]
pub struct StreamPageParams {
    pub hostname: String,
    pub src: String,
    pub file_name: String,
}
