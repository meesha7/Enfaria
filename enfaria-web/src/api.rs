use crate::prelude::*;

pub fn routes(app: &mut Server<State>) {
    app.at("api/login")
        .post(|req: Request<State>| async { login_fn(req).await });

    app.at("api/server").get(|_| async {
        Ok(Response::builder(200)
            .content_type(tide::http::mime::JSON)
            .body(env::var("SERVER").unwrap())
            .build())
    });

    app.at("api/version").get(|_| async {
        Ok(Response::builder(200)
            .content_type(tide::http::mime::JSON)
            .body(env::var("VERSION").unwrap())
            .build())
    });
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
    let query = sqlx::query("SELECT id, password FROM users WHERE username = ?")
        .bind(login.username)
        .fetch_one(pool)
        .await?;

    let db_password: Vec<u8> = query.try_get(1)?;
    let db_password: String = std::str::from_utf8(&db_password)?.to_string();

    bcrypt::verify(login.password, &db_password)?;

    let id: i64 = query.try_get(0)?;

    let query = sqlx::query("SELECT secret FROM sessions WHERE user_id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    let session_id: String = query.try_get(0)?;

    Ok(Response::builder(200)
        .content_type(tide::http::mime::JSON)
        .body(session_id)
        .build())
}
