use crate::prelude::*;

pub fn routes(
    _tera: Arc<Tera>,
    pool: Arc<MySqlPool>,
) -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::get()
        .and(warp::path("logout"))
        .and(warp::cookie("session-id"))
        .and(with_db(pool))
        .and_then(logout_fn)
}

async fn logout_fn(
    cookie: String,
    pool: Arc<MySqlPool>,
) -> Result<impl Reply, Rejection> {
    warp_unwrap!(
        sqlx::query("DELETE FROM sessions WHERE secret = ?")
            .bind(&cookie)
            .execute(pool.as_ref())
            .await
    );

    let domain = env::var("DOMAIN").unwrap();
    let remove_cookie = Cookie::build("session-id", "")
        .expires(OffsetDateTime::unix_epoch())
        .path("/")
        .domain(domain)
        .finish();

    Ok(warp::reply::with_header(
         warp::redirect(Uri::from_static("/")),
         "Set-Cookie",
         remove_cookie.to_string(),
    ))
}
