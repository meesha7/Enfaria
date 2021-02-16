use crate::prelude::*;

#[derive(Clone)]
pub struct Template {
    pub name: String,
    pub value: Context,
}

impl Template {
    pub fn new(name: &str) -> Self {
        Template {
            name: name.to_owned(),
            value: Context::new(),
        }
    }

    pub fn with_context(name: &str, context: Context) -> Self {
        Template {
            name: name.to_owned(),
            value: context,
        }
    }

    pub fn render(self, tera: &Tera) -> Response {
        let rendered = tera
            .render(&self.name, &self.value)
            .unwrap_or_else(|err| err.to_string());

        Response::builder(200)
            .body(rendered)
            .content_type(tide::http::mime::HTML)
            .build()
    }

    pub fn render_new(self) -> Response {
        let tera = Tera::new("templates/*").unwrap();
        self.render(&tera)
    }
}
