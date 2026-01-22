#![allow(dead_code)]

fn main() {
    // remember tuple structs are just named tuples

    struct Pair(i32, i32);

    #[derive(Debug, Clone)]
    struct Person {
        name: String,
        age: u8,
    }

    // the tuple struct have semi-colons at the end because of this
    // https://users.rust-lang.org/t/why-is-a-semicolon-required-at-the-end-of-a-tuple-struct-definition/25589/7

    let name = String::from("Brian");
    let age = 24;

    let person = Person { name, age };
    println!("Person = {:?}", person);

    // structs can be initialized with the values from another struct by using the ..EXPR expression
    // where EXPR can be another struct

    let person_1 = Person {
        name: String::from("Brian"),
        age: 25,
    };
    let person_2 = Person {
        name: String::from("David"),
        ..person_1.clone()
    };
    let person_3 = Person { ..person_1.clone() };

    // you must note that the ownership rules applies strongly here too
    // if the field be extracted from the parent struct does not implement Copy,
    // it would moved into the new struct the parent struct field would become invalid
    // so if you wish to use the parent struct after this process, you must clone the field
    // of implement Clone trait for that field type

    dbg!(person_1, person_2, person_3);

    // tuple struct are implicitly functions calls that returns the Struct instance
    // e.g

    struct Bound(i32, i32);

    // when you instantiate an instance of Bound, are you infact calling a function
    // fn Bound(elem0: i32, elem1: i32) -> Bound {}
}
