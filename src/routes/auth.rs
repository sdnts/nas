use anyhow::*;
use serde::Deserialize;
use sha2::{Digest, Sha512};
use tide::http::Cookie;

use crate::app_state::AppState;
use crate::db::NASDB;
use crate::schema::{Session, User};
use crate::templates::{AuthPageParams, FileListPageParams};

pub async fn get(req: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    let templates = req.state().clone().templates.clone();
    let response_body = templates.render(
        "auth",
        &AuthPageParams {
            title: "/auth".to_string(),
            hostname: "0zark".to_string(),
            message: "".to_string(),
        },
    )?;

    let response = tide::Response::builder(tide::StatusCode::Ok)
        .body(response_body)
        .content_type("text/html;charset=utf-8")
        .build();

    Ok(response)
}

#[derive(Debug, Deserialize)]
struct AuthPOSTParams {
    username: String,
    password: String,
}
pub async fn post(mut req: tide::Request<AppState>) -> Result<tide::Response, tide::Error> {
    let params: AuthPOSTParams = req.body_form().await?;
    let templates = req.state().clone().templates.clone();

    let mut hasher = Sha512::new();
    hasher.update(params.password);
    let password_hash = hasher.finalize();
    let password_hash = password_hash.as_slice().to_vec();
    let password_hash = hex::encode(&password_hash);

    let user = User::login(&params.username, &password_hash);
    println!("{:?}", &user);

    if let Ok(user) = user {
        // let db_session = Session::create(user.id)?;
        let session = req.session_mut();
        session
            .insert("user-id", user.id.to_string())
            .with_context(|| anyhow!("[auth::post] Failed to insert into session"))?;

        let response_body = templates.render(
            "auth",
            &AuthPageParams {
                title: "/auth".to_string(),
                hostname: "0zark".to_string(),
                message: "Logged in".to_string(),
            },
        )?;
        let response = tide::Response::builder(200)
            .body(response_body)
            .content_type("text/html;charset=utf-8")
            .build();

        Ok(response)
    } else {
        let response_body = templates.render(
            "auth",
            &AuthPageParams {
                title: "/auth".to_string(),
                hostname: "0zark".to_string(),
                message: "Invalid credentials".to_string(),
            },
        )?;

        let response = tide::Response::builder(tide::StatusCode::Ok)
            .body(response_body)
            .content_type("text/html;charset=utf-8")
            .build();

        Ok(response)
    }
}
