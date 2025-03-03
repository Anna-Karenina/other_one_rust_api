extern crate cr8s;

use rocket_db_pools::Database;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![
                cr8s::rocket_routes::options,
                cr8s::rocket_routes::authorization::login,
                cr8s::rocket_routes::authorization::me,
                cr8s::rocket_routes::rustaceans::get_rustaceans,
                cr8s::rocket_routes::rustaceans::get_rustacean,
                cr8s::rocket_routes::rustaceans::crate_rustacean,
                cr8s::rocket_routes::rustaceans::update_rustacean,
                cr8s::rocket_routes::rustaceans::delete_rustaceans,
                cr8s::rocket_routes::crates::get_crates,
                cr8s::rocket_routes::crates::get_crate,
                cr8s::rocket_routes::crates::crate_crate,
                cr8s::rocket_routes::crates::update_crate,
                cr8s::rocket_routes::crates::delete_crates,
            ],
        )
        .attach(cr8s::rocket_routes::Cors)
        .attach(cr8s::rocket_routes::DbConn::init())
        .attach(cr8s::rocket_routes::CacheConn::init())
        .launch()
        .await;
}
