use crate::prelude::*;

pub fn routes(app: &mut Server<State>) {
    app.at("register")
        .get(|req: Request<State>| async move { Ok(Template::new("register.tera").render(req.state().tera.as_ref())) });

    app.at("register").post(|req| async { register_fn(req).await });
}

#[derive(Clone, Debug, Deserialize)]
pub struct Register {
    email: String,
    username: String,
    password: String,
    password2: String,
}

fn email_valid(email: &str) -> bool {
    email.len() <= 100 && checkmail::validate_email(&email.to_string())
}

fn username_valid(username: &str) -> bool {
    if username.len() < 3 || username.len() > 50 {
        return false;
    }

    username.chars().filter(|&ch| !ch.is_ascii()).count() == 0
}

fn password_valid(password: &str) -> bool {
    if password.len() < 8 || password.len() > 300 {
        return false;
    }

    let upper_case = password.chars().filter(|&ch| ch.is_uppercase()).count() > 0;
    let number = password.chars().filter(|&ch| ch.is_numeric()).count() > 0;

    upper_case && number
}

async fn register_fn(mut request: Request<State>) -> tide::Result {
    let state = request.state().clone();
    let pool = state.pool.as_ref();
    let register: Register = request.body_form().await?;
    let response = sqlx::query!(
        "SELECT * FROM users WHERE username = ? OR email = ?",
        &register.username,
        &register.email
    )
    .fetch_optional(pool)
    .await?;

    let template = "register.tera";
    if !email_valid(&register.email) {
        return Err(error(template, "E-mail is not valid!"));
    }

    if !username_valid(&register.username) {
        return Err(error(template, "Username is not valid!"));
    }

    if response.is_some() {
        return Err(error(template, "User already exists!"));
    }

    if !password_valid(&register.password) {
        return Err(error(template, "Password is not valid!"));
    }

    if register.password != register.password2 {
        return Err(error(template, "Passwords do not match!"));
    }

    let hash = bcrypt::hash(&register.password, 11)?;

    sqlx::query!(
        "INSERT INTO users (username, password, email) VALUES (?, ?, ?)",
        &register.username,
        hash,
        &register.email
    )
    .execute(pool)
    .await?;

    Ok(Redirect::new("/").into())
}
