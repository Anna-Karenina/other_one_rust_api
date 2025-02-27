use std::error::Error;

use rocket::{http::Status, response::status::Custom};
use serde_json::{json, Value};

pub mod auth;
pub mod crates;
pub mod rustaceans;

#[derive(rocket_db_pools::Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", e);
    return Custom(Status::InternalServerError, json!("Error"));
}
