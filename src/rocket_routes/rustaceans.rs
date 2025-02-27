use crate::rocket_routes::{server_error, DbConn};
use crate::{
    models::{NewRustacean, Rustacean},
    repositories::RustaceanRepository,
};
use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{json, Json, Value},
};
use rocket_db_pools::Connection;

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find_all(&mut db, 100)
        .await
        .map(|r| json!(r))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/rustaceans/<id>")]
pub async fn get_rustacean(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find(&mut db, id)
        .await
        .map(|r| json!(r))
        .map_err(|e| match e {
            diesel::result::Error::NotFound => Custom(Status::NotFound, json!("Not found")),
            _ => server_error(e.into()),
        })
}

#[rocket::post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn crate_rustacean(
    mut db: Connection<DbConn>,
    new_rustacean: Json<NewRustacean>,
) -> Result<Custom<Value>, Custom<Value>> {
    RustaceanRepository::create(&mut db, new_rustacean.into_inner())
        .await
        .map(|r| Custom(Status::Created, json!(r)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    mut db: Connection<DbConn>,
    id: i32,
    rustacean: Json<Rustacean>,
) -> Result<Value, Custom<Value>> {
    RustaceanRepository::update(&mut db, id, rustacean.into_inner())
        .await
        .map(|r| json!(r))
        .map_err(|e| match e {
            diesel::result::Error::NotFound => Custom(Status::NotFound, json!("Not found")),
            _ => server_error(e.into()),
        })
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustaceans(
    mut db: Connection<DbConn>,
    id: i32,
) -> Result<NoContent, Custom<Value>> {
    RustaceanRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => Custom(Status::NotFound, json!("Not found")),
            _ => server_error(e.into()),
        })
}
