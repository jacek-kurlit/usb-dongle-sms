use std::collections::HashMap;

use chrono::{DateTime, Utc};
use reqwest::Result;
use serde::Serialize;

use crate::{SmsApiClient, SmsError};

pub struct AlcatelRestApiClient {
    client: reqwest::blocking::Client,
    host: String,
}

impl AlcatelRestApiClient {
    pub fn new(host: String) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            host,
        }
    }
    fn send_sms_single(
        &self,
        message: String,
        receiver: String,
    ) -> Result<HashMap<String, String>> {
        self.client
            .post(format!("{}/jrd/webapi?api=SendSMS", self.host))
            .header("Referer", format!("{}/default.html", self.host))
            .header(
                "Content-Type",
                "application/x-www-form-urlencoded; charset=UTF-8",
            )
            .json(&SendSmsRequest::new(message, receiver))
            .send()
            .expect("Mock endpoint should be available")
            .json::<HashMap<String, String>>()
    }
}

impl SmsApiClient for AlcatelRestApiClient {
    fn send_sms(
        &self,
        message: String,
        receivers: Vec<String>,
    ) -> core::result::Result<(), SmsError> {
        for receiver in receivers {
            if self
                .send_sms_single(message.clone(), receiver.clone())
                .is_err()
            {
                //FIXME: this is poor man's impelentation, termnimal or UI should decide how to display error
                println!("Failed to send SMS to {receiver}");
                return Err(SmsError::ConnectionFailure);
            }
        }
        Ok(())
    }
}

#[derive(Serialize)]
struct SendSmsRequest {
    jsonrpc: String,
    id: f32,
    method: String,
    params: SendSmsParams,
}

impl SendSmsRequest {
    fn new(message: String, receiver: String) -> Self {
        Self {
            jsonrpc: "2.0".to_owned(),
            id: 6.6,
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
    phone_number: String,
    #[serde(rename = "SMSTime")]
    sms_time: DateTime<Utc>,
}

impl SendSmsParams {
    fn new(message: String, receiver: String) -> Self {
        Self {
            sms_id: -1,
            sms_content: message,
            phone_number: receiver,
            sms_time: DateTime::default(),
        }
    }
}
