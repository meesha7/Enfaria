use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AppError {
    template: String,
    message: String,
}

impl std::error::Error for AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn error(template: &str, message: &str) -> tide::Error {
    tide::Error::new(
        200,
        AppError {
            template: template.to_owned(),
            message: message.to_owned(),
        },
    )
}

pub async fn handle_error(response: Response) -> tide::Result<Response> {
    if let Some(err) = response.downcast_error::<AppError>() {
        let mut context = Context::new();
        context.insert("error", &err.message);

        let template = Template::with_context(&err.template, context);
        return Ok(template.render_new());
    }

    if let Some(e) = response.error() {
        error!("Internal Server Error: {:?}", e);
        let mut context = Context::new();
        context.insert("error", "Internal server error occured.");
        let template = Template::with_context("index.tera", context);
        return Ok(template.render_new());
    }

    Ok(response)
}
