use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder, Result};
use serde::Deserialize;
use sha2::{Digest, Sha512};

use crate::app_state::AppState;
use crate::error::NASError;
use crate::schema::User;
use crate::templates::AuthPageParams;

#[derive(Debug, Deserialize)]
pub struct FormParams {
    username: String,
    password: String,
}

pub async fn post(
    params: web::Form<FormParams>,
    app_state: web::Data<AppState>,
    identity: Identity,
) -> Result<impl Responder> {
    let templates = &app_state.templates;

    let mut hasher = Sha512::new();
    hasher.update(&params.password);
    let password_hash = hasher.finalize();
    let password_hash = password_hash.as_slice().to_vec();
    let password_hash = hex::encode(&password_hash);

    let user = User::login(&params.username, &password_hash);

    if let Ok(user) = user {
        identity.remember(user.username.to_string());
        dbg!(&identity.identity());

        let response_body = templates
            .render(
                "auth",
                &AuthPageParams {
                    title: "/auth".to_string(),
                    hostname: "0zark".to_string(),
                    message: "Logged in".to_string(),
                },
            )
            .map_err(|_| NASError::TemplateRenderError { template: "auth" })?;

        Ok(HttpResponse::Ok()
            .header("Content-Type", "text/html;charset=utf-8")
            .body(response_body))
    } else {
        let response_body = templates
            .render(
                "auth",
                &AuthPageParams {
                    title: "/auth".to_string(),
                    hostname: "0zark".to_string(),
                    message: "Invalid credentials".to_string(),
                },
            )
            .map_err(|_| NASError::TemplateRenderError { template: "auth" })?;

        Ok(HttpResponse::Ok()
            .header("Content-Type", "text/html;charset=utf-8")
            .body(response_body))
    }
}
