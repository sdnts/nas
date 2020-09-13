use actix_web::{Responder, Result};

use crate::error::NASError;

pub async fn get() -> Result<impl Responder> {
    Ok(format!("/auth get"))
}
