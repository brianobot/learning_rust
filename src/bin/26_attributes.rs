// attributes are hint to the compiler and can be used as code generate hint too

// syntax for attributes
// #[]

#[allow(dead_code)] // this is an attribute

//inner attributes apply to the container there are applied to
// #![allow(dead_code)]

// outer attributes apply to the element following their declaration
// #[allow(dead_code)]

#[derive(Debug, Clone)] // this is another attribute
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let paul = Person { name: String::from("Paul"), age: 34 };
    println!("");
    println!("person: {:?}", paul);
    println!("");
}