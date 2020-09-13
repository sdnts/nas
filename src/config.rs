use serde::Serialize;

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize)]
pub enum NASTheme {
    Light,
    Dark,
}

#[derive(Debug, Serialize)]
pub struct NASConfig {
    pub fs_root: String,
    pub cookie_secret: String,
    pub theme: NASTheme,
}
