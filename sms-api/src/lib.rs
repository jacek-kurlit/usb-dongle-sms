pub fn send_sms(message: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending {message}");
    Ok(())
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
