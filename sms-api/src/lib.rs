#![feature(result_option_inspect)]
use std::error::Error;

mod alcatel;
pub trait SmsApiClient {
    fn send_sms(&self, message: String, receivers: Vec<String>) -> Result<(), SmsError>;
    fn get_sms_status(&self) -> Result<SmsStatus, SmsError>;
}

pub enum SmsApiConfig {
    Alcatel { host: String },
}

pub fn create_api_clint(config: SmsApiConfig) -> Box<dyn SmsApiClient> {
    match config {
        SmsApiConfig::Alcatel { host } => Box::new(crate::alcatel::AlcatelRestApiClient::new(host)),
    }
}

#[derive(Debug)]
pub enum SmsError {
    ConnectionFailure,
}

pub enum SmsStatus {
    Success,
    Failure,
    Pending,
}

impl Error for SmsError {}

impl std::fmt::Display for SmsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmsError::ConnectionFailure => write!(f, "Connection failed"),
        }
    }
}
