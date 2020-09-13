use actix_identity::Identity;
use actix_web::{http, web, HttpResponse, Responder, Result};
use serde::Deserialize;
use sha2::{Digest, Sha512};

use crate::app_state::AppState;
use crate::error::NASError;
use crate::schema::User;
use crate::templates::AuthPageParams;
use crate::CONFIG;

#[derive(Debug, Deserialize)]
pub struct FormParams {
    username: String,
    password: String,
    redirect_url: Option<String>,
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

        let response_body = templates
            .render(
                "auth",
                &AuthPageParams {
                    theme: CONFIG.theme.clone(),
                    message: Some("Logged in, redirecting...".to_string()),
                    logged_in: true,
                    redirect_url: params.redirect_url.to_owned(),
                },
            )
            .map_err(|_| NASError::TemplateRenderError { template: "auth" })?;

        Ok(HttpResponse::Ok()
            .header(http::header::CONTENT_TYPE, "text/html;charset=utf-8")
            .body(response_body))
    } else {
        let response_body = templates
            .render(
                "auth",
                &AuthPageParams {
                    theme: CONFIG.theme.clone(),
                    message: Some("Invalid credentials".to_string()),
                    logged_in: false,
                    redirect_url: None,
                },
            )
            .map_err(|_| NASError::TemplateRenderError { template: "auth" })?;

        Ok(HttpResponse::Ok()
            .header(http::header::CONTENT_TYPE, "text/html;charset=utf-8")
            .body(response_body))
    }
}
