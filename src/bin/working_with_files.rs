use std::{fs, io::Read};

fn main() {
    // Rust provides the std::fs module to perform i/o operations

    let file = fs::File::open("hello.txt");
    let mut content = String::new();

    file.expect("Could not real file").read_to_string(&mut content);

    
}