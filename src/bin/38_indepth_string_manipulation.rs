fn main() {
    // remember they are two basic string types in rust
    // &str - string slice (immutable reference to a sequence of UTF-8 Characters)
        // when to use
        /*
        - when you don;t need to modify the string data
        - when you want to pass reference to a string without taking ownership
        - for string literal
         */

    let literal = "Hello World!";
    println!("Literal: {literal}");

    // String - heap allocated string (mutable sequence of UTF-8 Characters stored on the heap)
    // when to use
    /*
    - when you need to owned the string,
    - when you need to modify the string
    - when you're working with user input or dynamically generated string
     */
    let mut owned_string = String::from("Hello ");
    owned_string.push_str("world!");
    println!("Owned String: {owned_string}");

    // converting between string types
    // from &str to String
    let _to_string = literal.to_string();
    let _to_string = String::from(literal);
    
    // from String to &str
    let _to_str = owned_string.as_str();
    let _to_str = &*owned_string;

    // converting from bytes array to String
    let bytes = &[72, 101, 108, 108, 111];

    let string_from_bytes = String::from_utf8(bytes.to_vec()).expect("Unable to convert bytes to string");
    println!("String from bytes: {string_from_bytes}");

    // converting numbers to string
    let number = 42;
    // this is a dynamically generated string, so it makes sense why it is in String format and not the &str type
    let num_str = number.to_string();
    println!("Number String: {num_str}");

    // Basic String operations
    // Concatenation

    // using the + operator
    let hello = String::from("Hello");
    let world = "World!";

    let greeting = hello + " " + world; // hello is moved so it can not be used again
    // world is a string slice and is not moved here since it is a reference
    println!("Greeting: {greeting}");

    // using the format macro
    // this doubles for string interpolation allowing variables to be internpolated into strings
    let hello = String::from("Hello");
    let greeting = format!("{} {}", hello, world);
    println!("{}", greeting);

    // reverse a string
    let original = "Hello Brian!";
    let reversed = original.chars().rev().collect::<String>();
    println!("Reversed: {reversed}, Original: {original}");

    // slicing string
    // let slice = &original[0..5];

}