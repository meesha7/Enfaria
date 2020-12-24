use crate::prelude::*;

pub fn routes(
    tera: Arc<Tera>,
    pool: Arc<MySqlPool>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let login = warp::get()
        .and(warp::path("login"))
        .and(with_tera(tera.clone()))
        .and(with_template(Template::new("login.tera")))
        .map(render);

    let do_login = warp::post()
        .and(warp::path("login"))
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::form())
        .and(with_db(pool))
        .and(with_tera(tera))
        .and_then(login_fn);

    login.or(do_login)
}

#[derive(Clone, Debug, Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}

async fn login_fn(login: LoginData, pool: Arc<MySqlPool>, tera: Arc<Tera>) -> Result<impl Reply, Rejection> {
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
        return Err(warp::reject::custom(IncorrectPassword));
    };

    let id: i64 = warp_unwrap!(query.try_get(0));

    warp_unwrap!(
        sqlx::query("DELETE FROM sessions WHERE user_id = ?")
            .bind(id)
            .execute(pool.as_ref())
            .await
    );

    let session_id = Uuid::new_v4();
    let expiry_date = Utc::now() + Duration::days(30);
    let expiry_two = OffsetDateTime::now_utc() + 30_i32.days();

    warp_unwrap!(
        sqlx::query("INSERT INTO sessions (user_id, secret, expiry_date) VALUES (?, ?, ?)")
            .bind(id)
            .bind(session_id.to_string())
            .bind(expiry_date.format("%F %T").to_string())
            .execute(pool.as_ref())
            .await
    );

    let domain = env::var("DOMAIN").unwrap();
    let cookie = Cookie::build("session-id", session_id.to_string())
        .expires(expiry_two)
        .path("/")
        .domain(domain)
        .finish();

    let mut template = Template::new("index.tera");
    template.value.insert("logged_in", &true);
    Ok(warp::reply::with_header(
        render(tera, template),
        "Set-Cookie",
        cookie.to_string(),
    ))
}
