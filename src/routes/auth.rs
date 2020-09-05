use anyhow::Result;
use askama::Template;

use crate::templates::AuthPage;

pub(crate) async fn get(_: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    let response_body = AuthPage {
        name: "0zark".to_string(),
    };
    let response_body = response_body.render()?;

    let response = tide::Response::builder(200)
        .body(response_body)
        .content_type("text/html;charset=utf-8")
        .build();

    Ok(response)
}

pub(crate) async fn post(req: tide::Request<()>) -> Result<tide::Response, tide::Error> {
    println!("{:?}", req);
    unimplemented!()
}
