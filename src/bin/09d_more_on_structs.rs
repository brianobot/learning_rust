#![allow(dead_code)]

fn main() {
    // remember tuple structs are just named tuples

    struct Pair(i32, i32);

    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    // the tuple struct have semi-colons at the end because of this
    // https://users.rust-lang.org/t/why-is-a-semicolon-required-at-the-end-of-a-tuple-struct-definition/25589/7

    let name = String::from("Brian");
    let age = 24;

    let person = Person {name, age};
    println!("Person = {:?}", person);


}