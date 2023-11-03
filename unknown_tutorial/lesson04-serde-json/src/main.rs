use std::fs::File;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    address: Address,
    phones: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    street: String,
    city: String,
}

fn main() {
    let data = r#" {
        "name": "John Doe", "age": 43,
        "address": {"street": "main", "city":"Downtown"},
        "phones":["27726550023"]
       } "#;
    let p: Person = serde_json::from_str(data).expect("data is invalid");
    println!("Person is: \n{:#?}", p);
    println!("----------------------");
    let fp = File::open("./person.json");
    if let Ok(fp2) = fp {
        let p: Person = serde_json::from_reader(fp2).expect("file is not valid");
        println!("Person from file is: \n{:#?}", p);
    }
    println!("Hello, world!");
}
