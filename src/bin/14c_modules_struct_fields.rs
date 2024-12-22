// Struct fields are private by default when access from outside the scope of the module they are declared in
// Public structs with private fields cannot be constructed using field
// However, structs with private fields can be created using
// public constructors


mod module_one {
    fn intro(person: &Person) -> String {
        return format!("Hello! My Name is {} {}", &person.first_name, &person.last_name);
    }

    #[derive(Debug, Default)]
    pub struct Person {
        pub first_name: String,
        pub last_name: String,
        _age: u8
    }

    impl Person {
        pub fn new() -> Self {
            Self {
                ..Default::default()
            }
        }

        pub fn greet(&self) -> String {
            self::intro(self)
        }
    }
}

use crate::module_one::Person;

fn main() {
    let mut person = module_one::Person::new();
    let person_2 = Person::new();
    println!("Person 1 = {:?}", person);
    println!("Person 2 = {:?}", person_2);

    person.first_name = "Brian".to_string();
    person.last_name = "Obot".to_string();

    println!("Person first name: {}", person.first_name);
    println!("Person last name: {}", person.last_name);

    // self can be used to refer to the current module
    // super can be used to refer to the parent module
    let greeting  = person.greet();
    println!("Greeting: {}", greeting);

}