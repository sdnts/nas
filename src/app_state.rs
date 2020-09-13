use anyhow::*;
use handlebars::Handlebars;

use crate::hbs_helpers;

#[derive(Debug)]
pub struct AppState {
    pub templates: Handlebars<'static>,
}

impl AppState {
    pub fn new() -> Result<Self> {
        let mut handlebars = Handlebars::new();
        handlebars.register_templates_directory(".hbs", "src/templates/")?;

        handlebars.register_helper("lowercase", Box::new(hbs_helpers::lowercase));
        handlebars.register_helper("filesize", Box::new(hbs_helpers::filesize));

        Ok(Self {
            templates: handlebars,
        })
    }
}
