use diesel::{pg::PgConnection, prelude::*};
use std::env;
use iron::{IronResult, Response, Request, status};


pub fn establish_connection() -> PgConnection {
    let data_base_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    PgConnection::establish(&data_base_url)
        .expect(&format!("Error connecting to {}", data_base_url))
}

// Serves a string to the user.  Try accessing "/".
pub fn hello(_: &mut Request) -> IronResult<Response> {
    let resp = Response::with((status::Ok, "Hello world!"));
    Ok(resp)
}

pub fn get_server_port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080)
}