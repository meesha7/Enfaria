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

#[derive(Debug)]
struct IncorrectPassword;
impl warp::reject::Reject for IncorrectPassword{}

#[derive(Debug)]
struct InvalidPassword;
impl warp::reject::Reject for InvalidPassword{}

#[derive(Debug)]
struct InvalidEmail;
impl warp::reject::Reject for InvalidEmail{}

#[derive(Debug)]
struct InvalidUsername;
impl warp::reject::Reject for InvalidUsername{}

#[derive(Debug)]
struct ExistingUser;
impl warp::reject::Reject for ExistingUser{}

#[derive(Debug)]
struct HashError;
impl warp::reject::Reject for HashError{}

fn email_valid(email: &String) -> bool{
    return email.len() <= 100 && checkmail::validate_email(email);
}

fn username_valid(username: &String) -> bool{
    if username.len() < 3 || username.len() > 50{
        return false;
    }
    return username.chars().filter(|&ch| !ch.is_ascii()).count() == 0;
}

fn password_valid(password: &String) -> bool{
    if password.len() < 8 || password.len() > 300{
        return false;
    }
    let upper_case = password.chars().filter(|&ch| ch.is_uppercase()).count() > 0;
    let number = password.chars().filter(|&ch| ch.is_numeric()).count() > 0;
    return upper_case && number;
}

async fn register_fn(register: Register, pool: Arc<MySqlPool>) -> Result<impl Reply, Rejection> {
    let row = warp_unwrap!(
        sqlx::query("SELECT * FROM users WHERE username = ? OR email = ?")
            .bind(&register.username)
            .bind(&register.email)
            .fetch_optional(pool.as_ref())
            .await
    );

    if !email_valid(&register.email){
        return Err(warp::reject::custom(InvalidEmail));
    }

    if !username_valid(&register.username){
        return Err(warp::reject::custom(InvalidUsername));
    }

    if row.is_some() {
        return Err(warp::reject::custom(ExistingUser));
    }

    if !password_valid(&register.password){
        return Err(warp::reject::custom(InvalidPassword));
    }

    if register.password != register.password2{
        return Err(warp::reject::custom(IncorrectPassword));
    }

    let hash = match bcrypt::hash(&register.password, 11) {
        Ok(h) => h,
        _ => return Err(warp::reject::custom(HashError)),
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
