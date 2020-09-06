use askama::Template;

#[derive(Template, Debug)]
#[template(path = "400.html")]
pub struct BadRequestPage {
    pub title: String,
    pub hostname: String,
    pub username: String,
}

#[derive(Template, Debug)]
#[template(path = "auth.html")]
pub struct AuthPage {
    pub title: String,
    pub hostname: String,
}

#[derive(Template, Debug)]
#[template(path = "fs.html")]
pub struct FileListPage {
    pub title: String,
    pub hostname: String,
    pub username: String,
    pub breadcrumbs: Vec<String>,
    pub file_names: Vec<String>,
    pub file_sizes: Vec<u64>,
    pub file_extensions: Vec<String>,
    pub file_types: Vec<String>,
    pub test: serde_json::value::Value,
}

#[derive(Template, Debug)]
#[template(path = "stream.html", escape = "none")]
pub struct StreamPage {
    pub hostname: String,
    pub src: String,
    pub file_name: String,
}
