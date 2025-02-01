use std::sync::Arc;
use warp::{reject, reply, Filter, Rejection};
mod structs;
mod Structs;


#[tokio::main]
async fn main() {
    let users: Arc<structs::structs::User> = Arc::new(init_users());
    let login_route = warp::path!("login").and(warp::post()).and(users.clone()).and(warp::body::json()).and_then(login_handler);
}
