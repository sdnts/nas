use actix_web::{Responder, Result};

use crate::error::NASError;

pub async fn post() -> Result<impl Responder> {
    Ok(format!("/auth post"))
}
