use crate::prelude::*;

#[derive(Clone)]
pub struct Template {
    pub name: &'static str,
    pub value: Context,
}

impl Template {
    pub fn new(name: &'static str) -> Self {
        Template {
            name,
            value: Context::new(),
        }
    }

    pub fn with_context(name: &'static str, context: Context) -> Self {
        Template { name, value: context }
    }

    pub fn render(self, tera: &Tera) -> Response {
        let rendered = tera
            .render(self.name, &self.value)
            .unwrap_or_else(|err| err.to_string());

        Response::builder(200)
            .body(rendered)
            .content_type(tide::http::mime::HTML)
            .build()
    }
}
