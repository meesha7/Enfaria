use crate::prelude::*;
use chrono::{NaiveDateTime, Utc};
use cookie::Cookie;
use std::env;
use time::{Duration, OffsetDateTime};

pub fn routes(app: &mut Server<State>) {
    app.at("/").get(|req: Request<State>| async {
        let template = Template::new("index.tera");
        if let Some(cookie) = req.cookie("session-id") {
            auth_index_fn(req, cookie.value(), template).await
        } else {
            Ok(template.render(req.state().tera.as_ref()))
        }
    });
}

async fn auth_index_fn(request: Request<State>, cookie: &str, mut template: Template) -> tide::Result {
    let state = request.state().clone();
    let tera = state.tera.as_ref();
    let pool = state.pool.as_ref();
    let domain = env::var("DOMAIN").unwrap();
    let remove_cookie = Cookie::build("session-id", "")
        .expires(OffsetDateTime::now_utc() - Duration::day())
        .path("/")
        .domain(domain)
        .finish();

    let response = sqlx::query!("SELECT user_id, expiry_date FROM sessions WHERE secret = ?", &cookie)
        .fetch_optional(pool)
        .await?;

    if response.is_none() {
        let mut response: Response = template.render(tera);
        response.insert_header("Set-Cookie", remove_cookie.to_string());
        return Ok(response);
    }

    let response = response.unwrap();
    let date: NaiveDateTime = response.expiry_date;

    if Utc::now().naive_utc() > date {
        sqlx::query!("DELETE FROM sessions WHERE secret = ?", &cookie)
            .execute(pool)
            .await?;
        let mut response: Response = template.render(tera);
        response.insert_header("Set-Cookie", remove_cookie.to_string());
        return Ok(response);
    }

    template.value.insert("logged_in", &true);
    Ok(template.render(tera))
}
