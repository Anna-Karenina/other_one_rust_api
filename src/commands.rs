use std::str::FromStr;

use chrono::{Datelike, Utc};
use diesel_async::{AsyncConnection, AsyncPgConnection};

use tera::{Context, Tera};

use crate::{
    auth::hash_password,
    mail::HtmlMailer,
    models::{NewUser, RoleCode},
    repositories::{CrateRepository, RoleRepository, UserRepository},
};

fn load_template_engine() -> Tera {
    Tera::new("templates/**/*.html").expect("Cannot load template engine")
}

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Cannot load DB url from env");

    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Cannot connect to pg")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut c = load_db_connection().await;
    let password = hash_password(password).unwrap();
    let new_user = NewUser { username, password };
    let role_enums = role_codes
        .iter()
        .map(|v| RoleCode::from_str(v.as_str()).unwrap())
        .collect();

    let user = UserRepository::create_user(&mut c, new_user, role_enums)
        .await
        .unwrap();
    println!("User created {:?}", user);
    let roles = RoleRepository::find_by_user(&mut c, &user).await.unwrap();
    println!("Roles assigned {:?}", roles);
}

pub async fn list_users() {
    let mut c = load_db_connection().await;

    let users = UserRepository::find_with_roles(&mut c).await.unwrap();
    for user in users {
        println!("Users created {:?}\n", user);
    }
}

pub async fn delete_user(user_id: i32) {
    let mut c = load_db_connection().await;
    UserRepository::delete_user(&mut c, user_id).await.unwrap();
}

pub async fn digest_send(email: String, hours_since: i32) {
    let mut c = load_db_connection().await;
    let tera = load_template_engine();
    let crates = CrateRepository::find_since(&mut c, hours_since)
        .await
        .unwrap();
    if crates.len() > 0 {
        let year = Utc::now().year();
        let mut context = Context::new();
        context.insert("crates", &crates);
        context.insert("year", &year);

        let smtp_host = std::env::var("SMTP_HOST").expect("Cannot load smtp host from env");
        let smtp_username =
            std::env::var("SMTP_USERNAME").expect("Cannot load smtp username from env");
        let smtp_password =
            std::env::var("SMTP_PASSWORD").expect("Cannot load smtp password from env");

        let mailer = HtmlMailer {
            tempalte_engine: tera,
            smtp_host,
            smtp_username,
            smtp_password,
        };
        mailer
            .send(
                &email,
                &String::from("Cra8s digest"),
                "email/digest.html",
                context,
            )
            .unwrap();
    }
}
