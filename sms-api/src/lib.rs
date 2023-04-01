pub fn send_sms(message: String, receivers: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending {message} to {} receivers", receivers.len());
    // other testing
    Ok(())
}
