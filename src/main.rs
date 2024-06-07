
#[derive(Debug)]
struct Contact {
  _first_name: String,
  _last_name: String,
  _display_name: String,
  _email: String,
  _phone_number: String,
}

fn main() {
    let person = Contact {
        _first_name: String::from("Jason"),
        _last_name: String::from("Ribble"),
        _display_name: String::from("Jason Ribble"),
        _email: String::from("example@.com"),
        _phone_number: String::from("123-456-7890"),
    };

    println!("{:?}", person);
}
