#![allow(unused_variables)]

// THis is a revisit of the ownership rule in Rust
/*
- Each value in Rust has an owner
- There can only be one owner at a time
- When the owner goes out of scope, the value is dropped
*/

fn main() {
    // the scope of a variable is the range within a program where an item is valid
    let string_value = "hello";

    {
        // greeting is not valid at this point
        let greetings = "Hello there!"; // greetings is valid at this point

        // greetings is still valid at this point
    } // ths scope ends here and the greetings stop being valid

    let mut name = String::from("Brian");

    name.push_str(" David");
    println!("Name: {name}");

    // a similar mental construct for the scope and validity of the owner name
    // can be implied as seen for the owne string_value above
}
