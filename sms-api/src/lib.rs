pub fn send_sms(message: String, receivers: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending {message} to {} receivers", receivers.len());
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_send_sms() {
        send_sms("Hello World".to_string(), vec![]).unwrap();
    }
}
