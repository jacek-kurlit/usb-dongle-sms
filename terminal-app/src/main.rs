use clap::{arg, command, Parser};
use sms_api::SmsApiConfig;

fn main() {
    let args = Args::parse();
    let api_client = sms_api::create_api_clint(SmsApiConfig::Alcatel {
        host: "http://192.168.1.1".to_string(),
    });
    let result = api_client.send_sms(args.message, vec!["785226509".to_string()]);
    match result {
        Ok(_) => println!("SMS sent"),
        Err(e) => println!("Failed to send SMS: {}", e),
    }
}

#[derive(Parser, Debug)]
#[command(name = "sms", author = "J.Kurlit", about, version)]
struct Args {
    #[arg(short, long)]
    message: String,
}
