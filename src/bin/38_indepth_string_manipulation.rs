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

    let string_from_bytes =
        String::from_utf8(bytes.to_vec()).expect("Unable to convert bytes to string");
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

    // word[0..2] is syntantic sugar for *word.get(0..0).unwrap()
    // so adding the & infront of a slice returns it to a string slice
    // let slice = &*original.get(0..2).unwrap();

    // Advanced string manipulation
    // string seaching and pattern matching

    let text = "The quick brown fox jumps over the lazy dog";

    if text.contains("fox") {
        println!("Found the found 'fox'!");
    }

    if let Some(index) = text.find("brown") {
        println!("'brown' starts at index {index}");
    }

    if text.starts_with("Hello") {
        println!("Text starts with hello");
    }

    if text.ends_with("dog") {
        println!("Text ends with dog");
    }

    let sentence = "apple,banana,grape,orange";
    let fruits = sentence.split(",").collect::<Vec<&str>>();
    println!("Fruits: {fruits:?}");

    let sentence = "The quick brown fox";
    // Split the string by whitespace
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("Words: {words:?}");

    // replacing part of a string
    let text = "I like dogs, Dogs are great!";
    let new_text = text.replace("dogs", "cats").replace("Dogs", "Cats");
    println!("{new_text}");

    // replace n first occurence of a string
    let new_text = text.replacen("dogs", "cats", 1);
    println!("{new_text}");

    // other methods
    // - trim(): removes leading and trailing whitespace
    // - trim_matches(&str): removes leading or trailing matches of &str
}
