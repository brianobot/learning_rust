

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
}