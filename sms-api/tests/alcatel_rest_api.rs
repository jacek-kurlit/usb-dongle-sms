use httpmock::prelude::*;
use serde_json::json;
use sms_api::*;

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
        "patrams": {
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
    let sms_api_client = sms_api::create_api_clint(SmsApiConfig::Alcatel {
        host: format!("http://{mock_address}"),
    });

    let result = sms_api_client.send_sms(message.to_string(), vec![phone_number.to_string()]);

    // then
    endpoint_mock.assert();
    println!("{}", endpoint_mock.hits());
    assert!(result.is_ok());
}
