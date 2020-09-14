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

impl Default for NASConfig {
    fn default() -> Self {
        Self {
            fs_root: "/home/ozark/nas_root".to_string(),
            cookie_secret: "012345678901234567890123456789012".to_string(),
            theme: NASTheme::Dark,
        }
    }
}
