use anyhow::*;
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
    let param = h
        .param(0)
        .ok_or(RenderError::new("Param 0 is required for format helper."))?;
    let param = format!("{}", param.value().render());
    out.write(&param.to_lowercase())?;
    Ok(())
}

impl AppState {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_templates_directory(".hbs", "src/templates/")
            .unwrap();

        handlebars.register_helper("lowercase", Box::new(lowercase));

        Self {
            templates: Arc::new(handlebars),
        }
    }
}
