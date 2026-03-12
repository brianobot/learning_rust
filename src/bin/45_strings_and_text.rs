fn main() {
    // Rust treats strings as UTF-8 Encoded by default
    let s = "你好 Rust";
    for (index, c) in s.chars().enumerate() {
        println!("{index}: {}", c);
    }

    for (index, i) in s.bytes().enumerate() {
        println!("{index}: {}", i as char);
    }
}
