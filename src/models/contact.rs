use super::{email::Email, phone_number::PhoneNumber};

#[derive(Debug)]
pub struct Contact {
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,
    pub email: Email,
    pub phone_number: PhoneNumber,
}

impl Contact {
    pub fn new(first_name: String, last_name: String, email: String, phone_number: String) -> Self {
        let display_name = format!("{first_name} {last_name}");
        let phone_number = PhoneNumber::new(phone_number).unwrap();
        let email = Email::new(email).unwrap();

        Self {
            first_name,
            last_name,
            display_name,
            email,
            phone_number,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Contact;

    #[test]
    fn test_display_name() {
        let person = Contact::new(
            String::from("Jason"),
            String::from("Ribble"),
            String::from("john@example.com"),
            String::from("123-456-7890"),
        );
        let display_name = "Jason Ribble".to_string();
        assert_eq!(person.display_name, display_name)
    }
}
