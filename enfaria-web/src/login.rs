use crate::prelude::*;
use chrono::{Duration, Utc};
use cookie::Cookie;
use time::{NumericalDuration, OffsetDateTime};
use uuid::Uuid;

pub fn routes(app: &mut Server<State>) {
    app.at("login")
        .get(|req: Request<State>| async move { Ok(Template::new("login.tera").render(req.state().tera.as_ref())) });

    app.at("login")
        .post(|req: Request<State>| async { login_fn(req).await });
}

#[derive(Clone, Debug, Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}

async fn login_fn(mut request: Request<State>) -> tide::Result {
    let state = request.state().clone();
    let pool = state.pool.as_ref();
    let login: LoginData = request.body_form().await?;
    let response = sqlx::query!("SELECT id, password FROM users WHERE username = ?", login.username)
        .fetch_one(pool)
        .await?;

    let db_password: Vec<u8> = response.password.into();
    let db_password: String = std::str::from_utf8(&db_password)?.to_string();
    bcrypt::verify(login.password, &db_password)?;

    let id: i32 = response.id;
    sqlx::query!("DELETE FROM sessions WHERE user_id = ?", id)
        .execute(pool)
        .await?;

    let session_id = Uuid::new_v4();
    let expiry_date = Utc::now() + Duration::days(30);
    let expiry_two = OffsetDateTime::now_utc() + 30_i32.days();

    sqlx::query!(
        "INSERT INTO sessions (user_id, secret, expiry_date) VALUES (?, ?, ?)",
        id,
        session_id.to_string(),
        expiry_date.format("%F %T").to_string()
    )
    .execute(pool)
    .await?;

    let cookie = Cookie::build("session-id", session_id.to_string())
        .expires(expiry_two)
        .finish();

    let mut response: Response = Redirect::new("/").into();
    response.insert_header("Set-Cookie", cookie.to_string());
    Ok(response)
}
