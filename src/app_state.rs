use handlebars::Handlebars;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub templates: Arc<Handlebars<'static>>,
}

impl AppState {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.register_templates_directory(".hbs", "src/templates/").unwrap();

        Self {
            templates: Arc::new(handlebars),
        }
    }
}
