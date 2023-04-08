use std::process::Command;

use chrono::{DateTime, Local};
use serde::Serialize;

use crate::{SmsApiClient, SmsError, SmsStatus::Success};

pub struct AlcatelRestApiClient {
    host: String,
}

impl AlcatelRestApiClient {
    pub fn new(host: String) -> Self {
        Self { host }
    }

    // NOTE: I could use reqwest or other http client but I enyoyed using system tools like curl too much :)
    // TODO: endpoint can handle up to 3 receivers at once it could be used to speed up sending
    fn send_sms(&self, message: String, receiver: String) -> Result<String, String> {
        // converst SendSmsRequest to json
        let body = serde_json::to_string(&SendSmsRequest::new(message, receiver)).unwrap();
        println!("body: {}", body);

        let output = Command::new("curl")
            .arg(format!("{}/jrd/webapi?api=SendSMS", self.host))
            .args(["-H", &format!("Referer: {}/default.html", self.host)])
            .args(["-H", "Content-Type: application/x-www-form-urlencoded"])
            .arg("--data-raw")
            .arg(body)
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            Ok(String::from_utf8(output.stdout).unwrap())
        } else {
            Err(String::from_utf8(output.stderr).unwrap())
        }
    }
}

impl SmsApiClient for AlcatelRestApiClient {
    fn send_sms(
        &self,
        message: String,
        receivers: Vec<String>,
    ) -> core::result::Result<(), SmsError> {
        for receiver in receivers {
            let resp = self.send_sms(message.clone(), receiver.clone());
            match resp {
                Ok(_) => {}
                Err(e) => {
                    println!("Failed to send SMS to {receiver}: {e}");
                    return Err(SmsError::ConnectionFailure);
                }
            }
        }
        Ok(())
    }

    fn get_sms_status(&self) -> core::result::Result<crate::SmsStatus, SmsError> {
        //TODO: implement
        Ok(Success)
    }
}

#[derive(Serialize)]
struct SendSmsRequest {
    jsonrpc: String,
    method: String,
    params: SendSmsParams,
    id: String,
}

impl SendSmsRequest {
    fn new(message: String, receiver: String) -> Self {
        Self {
            jsonrpc: "2.0".to_owned(),
            id: "6.6".to_owned(),
            method: "SendSMS".to_owned(),
            params: SendSmsParams::new(message, receiver),
        }
    }
}

#[derive(Serialize)]
struct SendSmsParams {
    #[serde(rename = "SMSId")]
    sms_id: i32,
    #[serde(rename = "SMSContent")]
    sms_content: String,
    #[serde(rename = "PhoneNumber")]
    phone_number: Vec<String>,
    #[serde(rename = "SMSTime")]
    sms_time: DateTime<Local>,
}

impl SendSmsParams {
    fn new(message: String, receiver: String) -> Self {
        Self {
            sms_id: -1,
            sms_content: message,
            phone_number: vec![receiver],
            sms_time: Local::now(),
        }
    }
}
