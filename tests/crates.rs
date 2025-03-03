use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_create_crate() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);

    let response = client
        .post(format!("{}/crates", common::APP_HOST))
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

    let u_crate: Value = response.json().unwrap();

    assert_eq!(
        u_crate,
        json!({
            "id": u_crate["id"],
            "rustacean_id": rustacean["id"],
            "code": "code",
            "name": "name_values",
            "version": "0.1",
            "description": "some description",
            "created_at": u_crate["created_at"]
        })
    );

    common::delete_test_crate(&client, u_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_get_crate() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);
    let u_crate = common::create_test_crate(&client, &rustacean);

    let response = client
        .get(format!("{}/crates/{}", common::APP_HOST, u_crate["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let u_crate: Value = response.json().unwrap();

    assert_eq!(
        u_crate,
        json!({
            "id": u_crate["id"],
            "rustacean_id": rustacean["id"],
            "code": "code",
            "name": "name_values",
            "version": "0.1",
            "description": "some description",
            "created_at": u_crate["created_at"]
        })
    );
    //Cleanup
    common::delete_test_crate(&client, u_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_crate() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);
    let rustacean2 = common::create_test_rustacean(&client);
    let u_crate = common::create_test_crate(&client, &rustacean);

    let response = client
        .put(format!("{}/crates/{}", common::APP_HOST, u_crate["id"]))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "code1",
            "name": "name_values1",
            "version": "0.2",
            "description": "some description1",
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let u_crate: Value = response.json().unwrap();

    assert_eq!(
        u_crate,
        json!({
            "id": u_crate["id"],
            "rustacean_id": rustacean["id"],
            "code": "code1",
            "name": "name_values1",
            "version": "0.2",
            "description": "some description1",
            "created_at": u_crate["created_at"]
        })
    );
    //Change user test and text
    let test_text =  "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?";
    let response = client
        .put(format!("{}/crates/{}", common::APP_HOST, u_crate["id"]))
        .json(&json!({
            "rustacean_id": rustacean2["id"],
            "code": "code1",
            "name": "name_values1",
            "version": "0.2",
            "description": test_text,
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let u_crate: Value = response.json().unwrap();

    assert_eq!(
        u_crate,
        json!({
            "id": u_crate["id"],
            "rustacean_id": rustacean2["id"],
            "code": "code1",
            "name": "name_values1",
            "version": "0.2",
            "description": test_text,
            "created_at": u_crate["created_at"]
        })
    );

    common::delete_test_crate(&client, u_crate);
    common::delete_test_rustacean(&client, rustacean);
    common::delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_delete_crate() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);
    let u_crate = common::create_test_crate(&client, &rustacean);

    let response = client
        .delete(format!("{}/crates/{}", common::APP_HOST, u_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_get_crates() {
    //SETUP
    let client = common::get_client_with_logged_in_admin();
    let rustacean = common::create_test_rustacean(&client);
    let u_crate = common::create_test_crate(&client, &rustacean);
    let b_crate = common::create_test_crate(&client, &rustacean);

    //TEST
    let respose = client
        .get(format!("{}/crates", common::APP_HOST))
        .send()
        .unwrap();

    assert_eq!(respose.status(), StatusCode::OK);
    let json: Value = respose.json().unwrap();
    assert!(json.as_array().unwrap().contains(&u_crate));
    assert!(json.as_array().unwrap().contains(&b_crate));

    //CLEANUP
    common::delete_test_crate(&client, u_crate);
    common::delete_test_crate(&client, b_crate);
    common::delete_test_rustacean(&client, rustacean);
}
