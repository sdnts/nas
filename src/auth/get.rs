use actix_web::{web, HttpResponse, Responder, Result};

use crate::app_state::AppState;
use crate::error::NASError;
use crate::templates::AuthPageParams;

pub async fn get(
    path: web::Path<String>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder> {
    let templates = &app_state.templates;

    let response_body = templates
        .render(
            "auth",
            &AuthPageParams {
                title: "/auth".to_string(),
                hostname: "0zark".to_string(),
                message: "".to_string(),
            },
        )
        .map_err(|_| NASError::TemplateRenderError { template: "auth" })?;

    Ok(HttpResponse::Ok()
        .body(response_body)
        .with_header("Content-Type", "text/html;charset=utf-8"))
}
