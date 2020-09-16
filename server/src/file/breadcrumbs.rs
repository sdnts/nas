use std::ffi::OsString;
use std::path::Path;

use crate::file::RelativePath;

pub type Breadcrumbs = Vec<String>;

impl From<&RelativePath> for Breadcrumbs {
    fn from(path: &RelativePath) -> Self {
        let relative_path = OsString::from(path);
        let relative_path = Path::new(&relative_path);

        let breadcrumbs: Vec<String> = relative_path
            .components()
            .map(|component| {
                // Must convert components to UTF8 to be able to display in the browser. We might lose a few characters
                component.as_os_str().to_string_lossy().to_string()
            })
            .collect();

        breadcrumbs
    }
}

// impl From<AbsolutePath> does not exist because we never want to be able to generate breadcrumbs from an absolute path
// (to avoid leaking the absolute path outside the server)
