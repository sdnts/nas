use anyhow::Result;
use serde_json::json;

use crate::app_state::AppState;
use crate::templates::AuthPage;

pub(crate) async fn get(req: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    let templates = req.state().clone().templates;
    let response_body = templates.render(
        "auth",
        &json!({
            "title": "/fs".to_string(),
            "hostname": "0zark".to_string()
        }),
    )?;

    let response = tide::Response::builder(200)
        .body(response_body)
        .content_type("text/html;charset=utf-8")
        .build();

    Ok(response)
}

pub(crate) async fn post(req: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    unimplemented!()
}
