use crate::prelude::*;
use listenfd::ListenFd;
use warp::hyper::server::Server;

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

/// For autoreload functionality, run the server as:
///     systemfd --no-pid -s http::8000 -- cargo watch -x 'run'
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

    let routes = api
        .or(login)
        .or(logout)
        .or(register)
        .or(statics)
        .or(index)
        .recover(recover::recover);

    let svc = warp::service(routes);

    let make_svc = hyper::service::make_service_fn(|_: _| {
        let svc = svc.clone();
        async move { Ok::<_, Infallible>(svc) }
    });

    let mut listenfd = ListenFd::from_env();

    let server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        Server::from_tcp(l).unwrap()
    } else {
        Server::bind(&([0, 0, 0, 0], 8000).into())
    };

    server.serve(make_svc).await.unwrap();
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
