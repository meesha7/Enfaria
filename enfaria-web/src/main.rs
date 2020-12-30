use crate::prelude::*;

#[macro_use]
pub mod macros;
pub use macros::*;

pub mod api;
pub mod error;
pub mod index;
pub mod login;
pub mod logout;
pub mod prelude;
pub mod recover;
pub mod register;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let tera = Arc::new(Tera::new("templates/*").unwrap());

    let pool = Arc::new(
        MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap(),
    );

    let statics = warp::path("static")
        .and(warp::fs::dir("./static/"))
        .with(warp::compression::gzip());

    let api = api::routes(tera.clone(), pool.clone());
    let index = index::routes(tera.clone(), pool.clone());
    let login = login::routes(tera.clone(), pool.clone());
    let logout = logout::routes(tera.clone(), pool.clone());
    let register = register::routes(tera.clone(), pool.clone());

    warp::serve(
        api.or(login)
            .or(logout)
            .or(register)
            .or(statics)
            .or(index)
            .recover(recover::recover),
    )
    .run(([0, 0, 0, 0], 8000))
    .await;
}

#[derive(Clone)]
pub struct Template {
    name: &'static str,
    value: Context,
}

impl Template {
    fn new(name: &'static str) -> Self {
        Template {
            name,
            value: Context::new(),
        }
    }

    fn with_context(name: &'static str, context: Context) -> Self {
        Template { name, value: context }
    }
}

pub fn render(tera: Arc<Tera>, template: Template) -> impl warp::Reply {
    let render = tera
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());
    warp::reply::html(render)
}

pub fn with_tera(tera: Arc<Tera>) -> impl Filter<Extract = (Arc<Tera>,), Error = Infallible> + Clone {
    warp::any().map(move || tera.clone())
}

pub fn with_db(db_pool: Arc<MySqlPool>) -> impl Filter<Extract = (Arc<MySqlPool>,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn with_template(template: Template) -> impl Filter<Extract = (Template,), Error = Infallible> + Clone {
    warp::any().map(move || template.clone())
}
