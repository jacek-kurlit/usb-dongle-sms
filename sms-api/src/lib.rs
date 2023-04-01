pub fn send_sms(message: String, receivers: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending {message} to {} receivers", receivers.len());
    Ok(())
}

pub fn name(_x: i32) -> i32 {
    0
}
