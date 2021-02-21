use crate::prelude::*;

pub fn routes(app: &mut Server<State>) {
    app.at("api/login")
        .post(|req: Request<State>| async { login_fn(req).await });
}

#[derive(Debug, Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}

async fn login_fn(mut req: Request<State>) -> tide::Result {
    let state = req.state().clone();
    let pool = state.pool.as_ref();
    let login: LoginData = req.body_form().await?;
    let response = sqlx::query!("SELECT id, password FROM users WHERE username = ?", login.username)
        .fetch_one(pool)
        .await?;

    let db_password: Vec<u8> = response.password.into();
    let db_password: String = std::str::from_utf8(&db_password)?.to_string();

    bcrypt::verify(login.password, &db_password)?;

    let id: i32 = response.id;
    let response = sqlx::query!("SELECT secret FROM sessions WHERE user_id = ?", id)
        .fetch_one(pool)
        .await?;

    let session_id: String = response.secret;

    Ok(Response::builder(200)
        .content_type(tide::http::mime::JSON)
        .body(session_id)
        .build())
}
