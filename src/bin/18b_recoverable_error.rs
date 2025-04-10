use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Result is a enum having two variants, Ok(T) and Err(E)
    // T and E are generic types and they would be discussed soon
    // because these have generic datatype we can use the result enum in alot of scenarios

    let _greeting_file_result = File::open("hello.txt");

    let greeting_file = match _greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => panic!("problem with opening file error: {:?}", error),
        },
    };
    // very verbose init

    println!("_Greeting File = {:?}", greeting_file);

    // we can achieve the same logic by using the unwrap result method
    let greeting_file = File::open("hello.txt").unwrap();

    println!("Greeting File = {:?}", greeting_file);

    // similarly the expect method allows us to customize the error message when the statement panics
    let _closing_file = File::open("closing.txt").expect("Error not found");
    // the expect method is used more in production by more RUSTACEANS

    // error propagation
}
