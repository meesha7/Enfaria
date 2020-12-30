use crate::prelude::*;
use std::env;

pub fn routes(
    tera: Arc<Tera>,
    pool: Arc<MySqlPool>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let login = warp::post()
        .and(warp::path("api"))
        .and(warp::path("login"))
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::form())
        .and(with_db(pool))
        .and(with_tera(tera))
        .and_then(login_fn);

    let getserver = warp::get()
        .and(warp::path("api"))
        .and(warp::path("getserver"))
        .and_then(getserver_fn);

    login.or(getserver)
}

#[derive(Clone, Debug, Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}

async fn login_fn(login: LoginData, pool: Arc<MySqlPool>, _tera: Arc<Tera>) -> Result<impl Reply, Rejection> {
    let query = warp_unwrap!(
        sqlx::query("SELECT id, password FROM users WHERE username = ?")
            .bind(login.username)
            .fetch_one(pool.as_ref())
            .await
    );
    let db_password: Vec<u8> = warp_unwrap!(query.try_get(1));
    let db_password: String = warp_unwrap!(std::str::from_utf8(&db_password)).to_string();
    let matches = warp_unwrap!(bcrypt::verify(login.password, &db_password));
    if !matches {
        return Err(warp::reject());
    };

    let id: i64 = warp_unwrap!(query.try_get(0));

    let query = warp_unwrap!(
        sqlx::query("SELECT secret FROM sessions WHERE user_id = ?")
            .bind(id)
            .fetch_one(pool.as_ref())
            .await
    );
    let session_id: String = query.get(0);

    Ok(warp::reply::json(&session_id))
}

async fn getserver_fn() -> Result<impl Reply, Rejection> {
    let server = env::var("SERVER").unwrap();
    Ok(warp::reply::json(&server))
}
