use askama::Template;

#[derive(Template, Debug)]
#[template(path = "400.html")]
pub struct BadRequestPage {}

#[derive(Template, Debug)]
#[template(path = "auth.html")]
pub struct AuthPage {
    pub name: String,
}

#[derive(Template, Debug)]
#[template(path = "fs.html")]
pub struct FileListPage {
    pub name: String,
    pub file_list: Vec<String>,
}

#[derive(Template, Debug)]
#[template(path = "stream.html", escape = "none")]
pub struct StreamPage {
    pub name: String,
    pub src: String,
}
