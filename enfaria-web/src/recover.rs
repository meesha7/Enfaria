use crate::prelude::*;

pub async fn recover(err: Rejection) -> Result<impl Reply, Infallible> {
	let title;
	let message;
	let template;

	if let Some(IncorrectPassword) = err.find() {
		title = "Incorrect Password";
		message = "The password you entered is incorrect.";
		template = "login.tera";
	} else if let Some(InvalidPassword) = err.find() {
		title = "Invalid Password";
		message = "The password you entered is invalid.";
		template = "register.tera";
	} else if let Some(InvalidEmail) = err.find() {
		title = "Invalid E-mail";
		message = "The e-mail you entered is invalid.";
		template = "register.tera";
	} else if let Some(InvalidUsername) = err.find() {
		title = "Invalid Username";
		message = "The username you entered is invalid.";
		template = "register.tera";
	} else if let Some(ExistingUser) = err.find() {
		title = "Username Taken";
		message = "The username you entered is taken.";
		template = "register.tera";
	} else if let Some(HashError) = err.find() {
		title = "Hash Error";
		message = "Error occured during password hashing.";
		template = "register.tera";
	} else {
		title = "Server Error";
		message = "Please try again later.";
		template = "index.tera";
	}

	let tera = Arc::new(Tera::new("templates/*").unwrap());

	let mut context = Context::new();
	context.insert("error", &true);
	context.insert("title", title);
	context.insert("message", message);

	let template = Template::with_context(template, context);

	Ok(render(tera, template))
}
