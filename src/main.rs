/// Returns a greeting message.
///
/// This function is used to demonstrate a simple "Hello, World!" message
/// and serves as an example for unit testing in Rust.
fn system_message() -> String {
    "Hello, World!".to_string()
}

fn main() {
    println!("{}", system_message());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_message_returns_correct_greeting() {
        let expected = "Hello, World!";
        assert_eq!(system_message(), expected, "The system_message function should return 'Hello, World!'");
    }
}
