use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;
#[test]
fn test_get_rustaceans() {
    //SETUP
    let client = common::get_client_with_logged_in_admin();
    let rustacean1 = common::create_test_rustacean(&client);
    let rustacean2 = common::create_test_rustacean(&client);

    //TEST
    let respose = client
        .get(format!("{}/rustaceans", common::APP_HOST))
        .send()
        .unwrap();

    assert_eq!(respose.status(), StatusCode::OK);
    let json: Value = respose.json().unwrap();
    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));

    //CLEANUP
    common::delete_test_rustacean(&client, rustacean1);
    common::delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_create_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let respose = client
        .post(format!("{}/rustaceans", common::APP_HOST))
        .json(&json!({
            "name": "Foo bar",
            "email": "foo@bar.com"
        }))
        .send()
        .unwrap();
    assert_eq!(respose.status(), StatusCode::CREATED);

    let rustacean: Value = respose.json().unwrap();
    assert_eq!(
        rustacean,
        json!({
            "id":rustacean["id"],
            "name": "Foo bar",
            "email": "foo@bar.com",
            "created_at":rustacean["created_at"],
        })
    );
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_get_rustacean() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);

    let response = client
        .get(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
            rustacean["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let rustacean: Value = response.json().unwrap();

    assert_eq!(
        rustacean,
        json!({
            "id":rustacean["id"],
            "name": "Foo bar",
            "email": "foo@bar.com",
            "created_at":rustacean["created_at"],
        })
    );

    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_rustacean() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);

    let response = client
        .put(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
            rustacean["id"]
        ))
        .json(&json!({"name": "Fuzz", "email":"fiiz@gmail.com"}))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let rustacean: Value = response.json().unwrap();

    assert_eq!(
        rustacean,
        json!({
            "id":rustacean["id"],
            "name": "Fuzz",
            "email": "fiiz@gmail.com",
            "created_at":rustacean["created_at"],
        })
    );
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_rustacean() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);

    let response = client
        .delete(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
            rustacean["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
