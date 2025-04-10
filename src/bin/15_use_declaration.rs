use std::io;

fn main() {
    // the use declaration is similar to the import syntax in other programming languages
    // it simplt provides a way to refer to fully qualified names (path) as a single name in the scope it is used

    let _handler = io::stdin();
    // instead of refering to the stdin as std::io::stdin, we use the use declaration to bring io into scope and use it directly

    let x = 32;
    println!("x = {x}");
    println!("x = {x}");
    println!("x = {x}");
    println!("x = {x}");
    println!("x = {x}");
    println!("x = {x}");

    let word = String::from("Information");
    println!("Word = {word}");
    println!("Word = {word}");
    println!("Word = {word}");
    println!("Word = {word}");
    println!("Word = {word}");
    println!("Word = {word}");
    println!("Word = {word}");
    println!("Word = {word}");
    // the repeated call to the macro with the same argument here is possible because
    // println marcos does not take ownership of its arguments, but only take references to them
}
