use crate::models::User;
use crate::rocket_routes::{server_error, DbConn, EditorUser};
use crate::{
    models::{Crate, NewCrate},
    repositories::CrateRepository,
};
use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{json, Json, Value},
};
use rocket_db_pools::Connection;

#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    CrateRepository::find_all(&mut db, 100)
        .await
        .map(|r| json!(r))
        .map_err(|e| server_error(e.into()))
}

#[rocket::get("/crates/<id>")]
pub async fn get_crate(
    mut db: Connection<DbConn>,
    id: i32,
    _user: User,
) -> Result<Value, Custom<Value>> {
    CrateRepository::find(&mut db, id)
        .await
        .map(|r| json!(r))
        .map_err(|e| match e {
            diesel::result::Error::NotFound => Custom(Status::NotFound, json!("Not found")),
            _ => server_error(e.into()),
        })
}

#[rocket::post("/crates", format = "json", data = "<new_crate>")]
pub async fn crate_crate(
    mut db: Connection<DbConn>,
    new_crate: Json<NewCrate>,
    _user: EditorUser,
) -> Result<Custom<Value>, Custom<Value>> {
    CrateRepository::create(&mut db, new_crate.into_inner())
        .await
        .map(|r| Custom(Status::Created, json!(r)))
        .map_err(|e| server_error(e.into()))
}

#[rocket::put("/crates/<id>", format = "json", data = "<u_crate>")]
pub async fn update_crate(
    mut db: Connection<DbConn>,
    id: i32,
    u_crate: Json<Crate>,
    _user: EditorUser,
) -> Result<Value, Custom<Value>> {
    CrateRepository::update(&mut db, id, u_crate.into_inner())
        .await
        .map(|r| json!(r))
        .map_err(|e| match e {
            diesel::result::Error::NotFound => Custom(Status::NotFound, json!("Not found")),
            _ => server_error(e.into()),
        })
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crates(
    mut db: Connection<DbConn>,
    id: i32,
    _user: EditorUser,
) -> Result<NoContent, Custom<Value>> {
    CrateRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => Custom(Status::NotFound, json!("Not found")),
            _ => server_error(e.into()),
        })
}
