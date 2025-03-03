use std::error::Error;

use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    response::status::Custom,
    Request,
};
use rocket_db_pools::Connection;

use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use serde_json::{json, Value};

use crate::{models::User, repositories::UserRepository};

pub mod authorization;
pub mod crates;
pub mod rustaceans;

#[derive(rocket_db_pools::Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

#[derive(rocket_db_pools::Database)]
#[database("redis")]

pub struct CacheConn(rocket_db_pools::deadpool_redis::Pool);

pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", e);
    return Custom(Status::InternalServerError, json!("Error"));
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = req
            .headers()
            .get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");
        if let Some(header_value) = token {
            let mut cache = req
                .guard::<Connection<CacheConn>>()
                .await
                .expect("Cannot connect to redis in request guard");

            let mut db = req
                .guard::<Connection<DbConn>>()
                .await
                .expect("Cannot connect to redis in request guard");

            let result = cache
                .get::<String, i32>(format!("sessions/{}", header_value[1]))
                .await;
            if let Ok(user_id) = result {
                if let Ok(user) = UserRepository::find_by_id(&mut db, user_id).await {
                    return Outcome::Success(user);
                }
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}
