#![allow(dead_code)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// fields in the struct share the mutabilty of the struct instance, if the struct instance is mutable, so are the fields
// we can not directly control the mutability of the field at a struct definition by specify mut keyword
// to control the mutability of a struct field, a Cell is used, but this is beyond this part of the lesson

#[derive(Default, Debug)]
struct Person {
    first_name: String,
    last_name: String,
    birth_year: u16,
    birth_month: u8,
}

fn new_person() -> Person {
    Person {
        ..Default::default()
    }
}

impl Person {
    fn new() -> Self {
        // the Self keyword is used to refer to the struct itself
        Self {
            ..Default::default()
        }
    }
}

fn main() {
    // we have the definition and instantiation of a struct
    // the definition controls the structure of the struct
    // while the instantiation create a concrete instance of that struct,
    // we can also create methods that are linked to instances of the struct

    // instantiate the struct here
    let mut user_1 = User {
        active: true,
        username: "brian".to_string(),
        email: "brian@dev.com".to_string(),
        sign_in_count: 0,
    };

    // .to_string() vs String::from()?
    // .to_string() is a trait method that is implemented for any type that implements the Display trait
    // String::from() is a function that is used to create a new String instance from a string literal or a string slice

    user_1.sign_in_count += 1;
    println!("Sign in count: {}", user_1.sign_in_count);

    let person_1 = new_person();
    println!("Person 1 = {:?}", person_1);

    let person_2 = Person::new();
    println!("Person 2 = {:?}", person_2);
}
