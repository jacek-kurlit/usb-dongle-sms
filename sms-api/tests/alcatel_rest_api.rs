use std::collections::HashMap;

use httpmock::prelude::*;
use serde_json::json;

#[test]
fn should_send_sms_via_alcatel_rest_api() {
    // given
    let server = MockServer::start();
    let mock_address = server.address();
    let message = "sms message";
    let phone_number = "123123123";
    let referer_header = format!("http://{mock_address}/default.html");
    let content_type_header = "application/x-www-form-urlencoded; charset=UTF-8";
    let body = json!({
        "jsonrpc": "2.0",
        "id": 6.6,
        "method": "SendSMS",
        "params": {
            "SMSId": -1,
            "SMSContent": message,
            "PhoneNumber": phone_number,
            "SMSTime": "2023-03-29 21:34:53"
    }
    });
    let endpoint_mock = server.mock(|when, then| {
        when.method(POST)
            .path("/jrd/webapi")
            .query_param("api", "SendSMS")
            .header("Referer", referer_header.to_string())
            .header("Content-Type", content_type_header.to_string())
            .json_body(body.clone());
        then.status(200)
            .json_body(json!({ "jsonrpc": "2.0", "id": "6.6" }));
    });

    // when
    let client = reqwest::blocking::Client::new();

    let url = format!("http://{mock_address}/jrd/webapi?api=SendSMS");
    let resp = client
        .post(url)
        .header("Referer", referer_header.to_string())
        .header("Content-Type", content_type_header)
        .body(body.to_string())
        .send()
        .expect("Mock endpoint should be available")
        .json::<HashMap<String, String>>()
        .expect("Resposne should be a JSON");

    // then
    endpoint_mock.assert();
    println!("{}", endpoint_mock.hits());
    assert_eq!(
        resp.get("id").expect("Id should be present in response"),
        "6.6"
    );
    assert_eq!(
        resp.get("jsonrpc")
            .expect("jsonrpc should be present in response"),
        "2.0"
    );
}
