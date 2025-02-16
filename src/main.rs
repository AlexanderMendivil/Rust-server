use std::sync::Arc;
use std::collections::HashMap;
use std::convert::Infallible;
use structs::structs::LoginRequest;
use warp::{reject, reply, Filter, Rejection};

use auth::{with_auth, Role};
use error::Error::*;

mod structs;
mod auth;
mod error;


type Result<T> = std::result::Result<T, error::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;
type Users = Arc<HashMap<String, structs::structs::User>>;

#[tokio::main]
async fn main() {
    let users: Arc<structs::structs::User> = Arc::new(init_users());
    let login_route = warp::path!("login")
    .and(warp::post())
    .and(with_users(users.clone()))
    .and(warp::body::json())
    .and_then(login_handler);


    let user_route = warp::path!("user").and(with_auth(Role::user)).and_then(user_handler);
    let admin_route = warp::path!("admin").and(with_auth(Role::admin)).and_then(admin_handler);

    let routes = login_route.or(user_route).or(admin_route).recover(Error::handle_rejection);    
    warp::serve(routes).run(([127,0,0,1], 80000)).await;
}

fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
    warp::any().map(move || users.clone())
}

pub async fn login_handler(users: Users, Body: LoginRequest) -> WebResult<impl Ready>{
    match users.iter().find(|(_uid, user)| user.email == body.email && user.password == body.password);
}