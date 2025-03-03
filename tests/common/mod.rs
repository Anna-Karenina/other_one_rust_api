use std::process::Command;

use reqwest::{
    blocking::{Client, ClientBuilder},
    header, StatusCode,
};
use serde_json::{json, Value};

pub static APP_HOST: &'static str = "http://127.0.0.1:8000";

pub fn delete_test_rustacean(client: &Client, rustacean: Value) {
    let response = client
        .delete(format!("{}/rustaceans/{}", APP_HOST, rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn create_test_rustacean(client: &Client) -> Value {
    let respose = client
        .post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({
            "name": "Foo bar",
            "email": "foo@bar.com"
        }))
        .send()
        .unwrap();
    assert_eq!(respose.status(), StatusCode::CREATED);

    respose.json().unwrap()
}

pub fn delete_test_crate(client: &Client, u_crate: Value) {
    let response = client
        .delete(format!("{}/crates/{}", APP_HOST, u_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn create_test_crate(client: &Client, rustacean: &Value) -> Value {
    print!("rustacean id: {}", rustacean["id"]);
    let response = client
        .post(format!("{}/crates", APP_HOST))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "code",
            "name": "name_values",
            "version": "0.1",
            "description": "some description",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()
}

pub fn get_client_with_logged_in_admin() -> Client {
    let _ = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("test_user")
        .arg("1234")
        .arg("admin")
        .output()
        .unwrap();

    let client = Client::new();

    let response = client
        .post(format!("{}/login", APP_HOST))
        .json(&json!({
            "username":"test_user",
            "password":"1234"
        }))
        .send()
        .unwrap();

    //Un Auth test
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    let header_value = format!("Bearer {}", json["token"].as_str().unwrap());

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&header_value).unwrap(),
    );

    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}
