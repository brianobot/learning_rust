fn main() {
    // Rust treats strings as UTF-8 Encoded by default
    let s = "你好 Rust";
    for (index, c) in s.chars().enumerate() {
        println!("{index}: {}", c);
    }

    for (index, i) in s.bytes().enumerate() {
        println!("{index}: {}", i as char);
    }

    // given a String, you can display it UTF-8 bytes with the bytes method
    let text = String::from("Hello Brian");
    let bytes = text.as_bytes();
    for byte in bytes {
        println!("{}", byte);
    }

    // Char are 32 bit (4 bytes) types that represent a single character
    // methods available on char
    // .is_numeric()
    // .is_alphabetic()
    // .is_alphanumeric()
    // .is_whitespace()
    // .is_control()
    //
    // Since u8 overlap over the first 128 Ascii characters
    // rust provides char methods on u8 types prepended with is_ascii_
    // example of these methods include, these methods are also available on char too
    // .is_ascii_alphabetic() ...
    //
}
