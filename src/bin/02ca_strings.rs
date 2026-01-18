

fn main() {
    let speech = "\"Ouch!\" said the well.\n";
    
    println!("Speech: {speech}");
    
    // raw strings allow backslash  to be used within the quotes of a string without escaping them
    let raw_string = r"\\Some-information";
    println!("Raw String: {raw_string}");
    
    // if you want to include double quotes in your strings, you can use pounds signs with raw strings
    let raw = r###"Some Raw Information: "" "###;
    println!("Raw: {raw}"); // notice how the pounds signs count at the start must match that at the end
    // the number of pounds sign used does not matter
    // 
    let another_raw = r#"Some Information " iN the middle of the sea"#;
    println!("ANother Raw: {another_raw}");
    
    // bytes strings are prefixed with b
    let bytes = b"GET"; // the values are stored as a slice of u8
    println!("Bytes: {bytes:?}");
    // this are very different from regular strings, infact the only real similarity to strings is the way they are written
    // it is possible to have raw byte strings like so
    let raw_bytes = br"rief342";
    println!("Raw Bytes String: {raw_bytes:?}");
    
    // A String is a resizable buffer that holds well formed UTF-8 text
    // a String is actually a Vector with a guarantee that each item is a valid UTF-8 text
    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    
    // a &str pronouced stir is a reference to a run of UTF-8 text owned by someone else
    // you can think of a &str as nothing more than a &[u8] that is guarantee to hold UTF-8 text
    // &str can not be modified
    // a string literal is a &str created and stored in the code of the program and last for the execution of the program
    // let mut s = "Hello";
    // s[0usize] = 'c'; // this would not work
    // 
    // the len method on String and &str return the length in bytes not characters
    // if you really want the length in chars, use .chars().count() methods chain
    
    // ways to create a string
    // from &str
    let _name = "Brian David Obot".to_string();
    
    // to_owned: does the same thign, would be explained later
    let _nick_name = "Brian".to_owned();
    
    // format!() macro works like the println! macro but returns a String and it automatically adds a new line character
    // vectors of strings, slices, arrays have two methods that return a string from parts
    // concat() and join(sep)
    // 
    let name = "Prian";
    let corrected_name = name.replace("P", "B");
    println!("Old Name: {name}");
    println!("Corrected Name: {}", corrected_name);
    
    // string have methods like contains, 
    let contains_peanuts = "peanut".contains("nut");
    println!("Contains Nut: {contains_peanuts}");
    
    // other string-like types
    #[allow(unused_imports)]
    use std::path::{PathBuf, Path}; // use this when working with filenames
    // use Vec<u8> or &[u8] when working with binary data that isn't utf-8 encoded
    #[allow(unused_imports)]
    use std::ffi::{OsString, OsStr};// use this when working with environment variables
    #[allow(unused_imports)]
    use std::ffi::{CString, CStr}; // when working with C libraries and null terminated strings
    
    
}