use std::io;
use std::io::Write;


// mimicing the input function available in python here
fn input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}

fn main() {
    let result = input("What is your name? ");
    println!("Result = {}", result.unwrap());
}