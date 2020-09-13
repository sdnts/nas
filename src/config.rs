use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum NASTheme {
    Light,
    Dark,
    Auto,
}

#[derive(Debug, Serialize)]
pub struct NASConfig {
    pub fs_root: String,
    pub cookie_secret: String,
    pub theme: NASTheme,
}
