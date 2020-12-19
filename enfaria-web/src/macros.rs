#[derive(Debug)]
pub struct ServerError(pub String);

impl warp::reject::Reject for ServerError {}

macro_rules! warp_unwrap {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return Err(warp::reject::custom(ServerError(e.to_string()))),
        }
    };
}
