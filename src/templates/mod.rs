use serde::Serialize;

use crate::config::NASTheme;
use crate::file::NASFile;

#[derive(Serialize)]
pub struct BadRequestPageParams {
    pub theme: NASTheme,
    pub title: String,
    pub username: String,
}

#[derive(Serialize)]
pub struct AuthPageParams {
    pub theme: NASTheme,
    pub message: Option<String>,
    pub logged_in: bool,
    pub redirect_url: Option<String>,
}

#[derive(Serialize)]
pub struct FSPageParams {
    pub theme: NASTheme,
    pub username: String,
    pub breadcrumbs: Vec<String>,
    pub parent_href: String,
    pub files: Vec<NASFile>,
}

#[derive(Serialize)]
pub struct StreamPageParams {
    pub theme: NASTheme,
    pub src: String,
    pub file_name: String,
}
