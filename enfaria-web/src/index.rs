use crate::prelude::*;

pub fn routes(
    tera: Arc<Tera>,
    pool: Arc<MySqlPool>,
) -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    let auth_index = warp::get()
        .and(warp::path::end())
        .and(warp::cookie("session-id"))
        .and(with_tera(tera.clone()))
        .and(with_template(Template::new("index.tera")))
        .and(with_db(pool))
        .and_then(auth_index_fn);

    let index = warp::get()
        .and(warp::path::end())
        .and(with_tera(tera))
        .and(with_template(Template::new("index.tera")))
        .map(render);

    auth_index.or(index)
}

async fn auth_index_fn(
    cookie: String,
    tera: Arc<Tera>,
    mut template: Template,
    pool: Arc<MySqlPool>,
) -> Result<impl Reply, Rejection> {
    let domain = env::var("DOMAIN").unwrap();
    let reply = render(tera.clone(), template.clone());
    let remove_cookie = Cookie::build("session-id", "")
        .expires(OffsetDateTime::now_utc() - TimeDuration::day())
        .path("/")
        .domain(domain)
        .finish();

    let row = warp_unwrap!(
        sqlx::query("SELECT user_id, expiry_date FROM sessions WHERE secret = ?")
            .bind(&cookie)
            .fetch_optional(pool.as_ref())
            .await
    );

    if row.is_none() {
        return Ok(warp::reply::with_header(reply, "Set-Cookie", remove_cookie.to_string()));
    }

    let row = row.unwrap();
    let date: DateTime<Utc> = warp_unwrap!(row.try_get(1));

    if Utc::now() > date {
        warp_unwrap!(
            sqlx::query("DELETE FROM sessions WHERE secret = ?")
                .bind(&cookie)
                .execute(pool.as_ref())
                .await
        );
        return Ok(warp::reply::with_header(reply, "Set-Cookie", remove_cookie.to_string()));
    }

    template.value.insert("logged_in", &true);
    Ok(warp::reply::with_header(render(tera, template), "", ""))
}
