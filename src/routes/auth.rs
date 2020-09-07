use anyhow::Result;

use crate::app_state::AppState;
use crate::templates::AuthPageParams;

pub async fn get(req: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    let templates = req.state().clone().templates;
    let response_body = templates.render(
        "auth",
        &AuthPageParams {
            title: "/auth".to_string(),
            hostname: "0zark".to_string(),
        },
    )?;

    let response = tide::Response::builder(tide::StatusCode::Ok)
        .body(response_body)
        .content_type("text/html;charset=utf-8")
        .build();

    Ok(response)
}

pub async fn post(_: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    unimplemented!()
}
