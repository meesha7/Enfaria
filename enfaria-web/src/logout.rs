use crate::prelude::*;

pub fn routes(app: &mut Server<State>) {
    app.at("logout").get(|req: Request<State>| async {
        if let Some(cookie) = req.cookie("session-id") {
            logout_fn(req, cookie.value()).await
        } else {
            Ok(Redirect::new("/").into())
        }
    });
}

async fn logout_fn(request: Request<State>, cookie: &str) -> tide::Result {
    let state = request.state().clone();
    let pool = state.pool.as_ref();
    sqlx::query("DELETE FROM sessions WHERE secret = ?")
        .bind(&cookie)
        .execute(pool)
        .await?;

    let domain = env::var("DOMAIN").unwrap();
    let remove_cookie = Cookie::build("session-id", "")
        .expires(OffsetDateTime::unix_epoch())
        .path("/")
        .domain(domain)
        .finish();

    let mut response: Response = Redirect::new("/").into();
    response.insert_header("Set-Cookie", remove_cookie.to_string());
    Ok(response)
}
