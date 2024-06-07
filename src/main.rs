#[derive(Debug)]
struct Contact {
    first_name: String,
    last_name: String,
    _display_name: String,
    _email: String,
    _phone_number: String,
}

impl Contact {
    fn new(first_name: String, last_name: String, display_name: String, email: String, phone_number: String) -> Result<Contact, String> {
        if first_name.to_lowercase() == "ERROR" {
            return Err("First name cannot be 'ERROR'".to_string());
        }

        Ok(Contact {
            first_name,
            last_name,
            _display_name: display_name,
            _email: email,
            _phone_number: phone_number,
        })
    }
}

fn main() {
    let person_result = Contact::new(
        String::from("Jason"),
        String::from("Ribble"),
        String::from("Jason Ribble"),
        String::from("example@.com"),
        String::from("123-456-7890"),
    );

    match person_result {
        Ok(person) => println!("Hi, my name is {} {}", person.first_name, person.last_name),
        Err(error) => println!("Error creating contact: {}", error),
    }
}