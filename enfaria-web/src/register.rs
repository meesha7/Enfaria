use crate::prelude::*;

pub fn routes(
    tera: Arc<Tera>,
    pool: Arc<MySqlPool>,
) -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    let register = warp::get()
        .and(warp::path("register"))
        .and(with_tera(tera))
        .and(with_template(Template::new("register.tera")))
        .map(render);

    let do_register = warp::post()
        .and(warp::path("do_register"))
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::form())
        .and(with_db(pool))
        .and_then(register_fn);

    register.or(do_register)
}

#[derive(Clone, Debug, Deserialize)]
pub struct Register {
    email: String,
    username: String,
    password: String,
    password2: String,
}

async fn register_fn(register: Register, pool: Arc<MySqlPool>) -> Result<impl Reply, Rejection> {
    let row = warp_unwrap!(
        sqlx::query("SELECT * FROM users WHERE username = ? OR email = ?")
            .bind(&register.username)
            .bind(&register.email)
            .fetch_optional(pool.as_ref())
            .await
    );

    if row.is_some() {
        return Err(warp::reject::reject());
    }

    if register.password != register.password2{
        return Err(warp::reject::reject());
    }

    let hash = match bcrypt::hash(&register.password, 11) {
        Ok(h) => h,
        _ => return Err(warp::reject::reject()),
    };

    warp_unwrap!(
        sqlx::query("INSERT INTO users (username, password, email) VALUES (?, ?, ?)")
            .bind(&register.username)
            .bind(hash)
            .bind(&register.email)
            .execute(pool.as_ref())
            .await
    );

    Ok(warp::reply::html(""))
}
