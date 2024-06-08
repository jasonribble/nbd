use std::fmt::Display;

use regex::Regex;

#[derive(Debug)]
struct Contact {
    _first_name: String,
    _last_name: String,
    display_name: String,
    _email: String,
    phone_number: PhoneNumber,
}

#[derive(Debug)]
struct PhoneNumber(String);

impl PhoneNumber {
    fn new(phone_number: String) -> Result<Self, String> {
        if is_valid_phone_number(&phone_number) {
            Ok(Self(phone_number))
        } else {
            Err("Invalid phone number format".to_string())
        }
    }
}

impl Display for PhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn is_valid_phone_number(phone: &str) -> bool {
    let phone_regex = Regex::new(
        r"^\+?1?\s*(\(\d{3}\)|\d{3})[-.\s]*\d{3}[-.\s]*\d{4}(?:\s*(?:ext|x|ex)\.?\s*\d+)?$",
    )
    .unwrap();
    phone_regex.is_match(phone)
}

impl Contact {
    fn new(
        first_name: String,
        last_name: String,
        email: String,
        phone_number: String,
    ) -> Self {
        let display_name = format!("{first_name} {last_name}");

        let phone_number = PhoneNumber::new(phone_number).unwrap();

        Self {
            _first_name: first_name,
            _last_name: last_name,
            display_name,
            _email: email,
            phone_number,
        }
    }
}

fn main() {
    let person = Contact::new(
        String::from("Jason"),
        String::from("Ribble"),
        String::from("Jason Ribble"),
        String::from("123-456-7890"),
    );

    println!("Hi, my name is {}", person.display_name);
    println!("My phone number is {}", person.phone_number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_name() {
        let person = Contact::new(
            String::from("Jason"),
            String::from("Ribble"),
            String::from("example@.com"),
            String::from("123-456-7890"),
        );
        let display_name = "Jason Ribble".to_string();
        assert_eq!(person.display_name, display_name)
    }

    #[test]
    fn test_valid_phone_numbers() {
        let valid_numbers = [
            "1234567890",
            "123-456-7890",
            "123.456.7890",
            "(123) 456-7890",
            "+1 (123) 456-7890",
            "123-456-7890 ext. 1234",
            "123.456.7890 x1234",
            "(123) 456-7890 ex 1234",
            "1234567890x123",
        ];

        for number in &valid_numbers {
            assert!(
                is_valid_phone_number(number),
                "Valid number '{}' failed validation",
                number
            );
        }
    }

    #[test]
    fn test_invalid_phone_numbers() {
        let invalid_numbers = [
            "123456789",
            "1234567890123",
            "123-456-789",
            "(123 456-7890",
            "123.456.78901",
            "123-456-7890 ext",
            "123.456.7890 x",
            "(123) 456-7890 ex abc",
        ];

        for number in &invalid_numbers {
            assert!(
                !is_valid_phone_number(number),
                "Invalid number '{}' passed validation",
                number
            );
        }
    }
}
