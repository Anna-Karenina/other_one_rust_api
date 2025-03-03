use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};
use std::process::Command;

pub mod common;

#[test]
fn test_login() {
    Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("test_admin")
        .arg("1234")
        .arg("admin")
        .output()
        .unwrap();

    let client = Client::new();

    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "username":"test_admin",
            "password":"1234"
        }))
        .send()
        .unwrap();

    //Un Auth test
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().unwrap().len(), 128);

    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "username":"test_admin",
            "password":"12345"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
