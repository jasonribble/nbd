#[derive(Debug)]
struct Contact {
    _first_name: String,
    _last_name: String,
    display_name: String,
    _email: String,
    _phone_number: String,
}

impl Contact {
    fn new(first_name: String, last_name: String, email: String, phone_number: String) -> Result<Contact, String> {
        let display_name = format!("{} {}", first_name, last_name);

        Ok(Contact {
            _first_name: first_name,
            _last_name: last_name,
            display_name,
            _email: email,
            _phone_number: phone_number,
        })
    }
}

fn main() {
    let person = Contact::new(
        String::from("Jason"),
        String::from("Ribble"),
        String::from("Jason Ribble"),
        String::from("example@.com"),
    ).unwrap();

    println!("Hi, my name is {}", person.display_name);
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
        ).unwrap();

        let display_name = "Jason Ribble".to_string();
        assert_eq!(person.display_name, display_name)
    }
}