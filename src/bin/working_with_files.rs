use std::fs;

fn main() {
    // Rust provides the std::fs module to perform i/o operations

    let file = fs::File::open("hello.txt");
    // let mut content = String::new();

    let real_file = if let Ok(file) = file {
        file
    } else {
        let file = fs::File::create("new_file_create.bin");
        file.unwrap()
    };

    println!("real file: {:?}", real_file)
}
