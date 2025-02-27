use crate::{
    auth::{authorize_user, Credentials},
    repositories::{SessionRepository, UserRepository},
    rocket_routes::{server_error, CacheConn, DbConn},
};
use rocket::serde::json::{json, Json, Value};
use rocket::{http::Status, response::status::Custom};
use rocket_db_pools::Connection;

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    mut db: Connection<DbConn>,
    mut cache: Connection<CacheConn>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    let user = UserRepository::find_by_name(&mut db, &credentials.username)
        .await
        .map_err(|e| server_error(e.into()))?;

    let session_id = authorize_user(&user, credentials.into_inner())
        .map_err(|_| Custom(Status::Unauthorized, json!("Wrong credentials")))?;

    SessionRepository::create(&mut cache, session_id.clone(), user.id)
        .await
        .map_err(|e| server_error(e.into()))?;

    Ok(json!({
        "token": session_id
    }))
}
