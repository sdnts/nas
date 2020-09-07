use anyhow::*;
use bytesize::ByteSize;
use handlebars::{Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub templates: Arc<Handlebars<'static>>,
}

fn lowercase(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).ok_or(RenderError::new(
        "Param 0 is required for `lowercase` helper.",
    ))?;
    let param = format!("{}", param.value().render());
    out.write(&param.to_lowercase())?;
    Ok(())
}

fn filesize(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let param = h.param(0).ok_or(RenderError::new(
        "Param 0 is required for `filesize` helper.",
    ))?;
    if param.value().is_number() {
        let size = param.value().as_u64().ok_or(RenderError::new(
            "Param `filesize` helper must be a positive number",
        ))?;

        if size > 0 {
            out.write(&ByteSize::b(size).to_string())?;
        }

        Ok(())
    } else {
        Err(RenderError::new("Param `filesize` helper must be a number"))
    }
}

impl AppState {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_templates_directory(".hbs", "src/templates/")
            .unwrap();

        handlebars.register_helper("lowercase", Box::new(lowercase));
        handlebars.register_helper("filesize", Box::new(filesize));

        Self {
            templates: Arc::new(handlebars),
        }
    }
}
