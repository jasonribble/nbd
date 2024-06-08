mod types;
use crate::types::Contact;

fn main() {
    let person = Contact::new(
        String::from("Jason"),
        String::from("Ribble"),
        String::from("john@example.com"),
        String::from("123-456-7890"),
    );

    println!("Hi, my name is {}", person.display_name);
    println!("My phone number is {}", person.phone_number);
    println!("My email is {}", person.email);
}
