use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please guess the number: ");

    let mut guess = String::new();

    io::stdin()
        // the &mut guess is a reference to the guess variable, more on that later (according to this book i am reading) 
        // References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You Guess: {guess}");


}

// Important take aways from the code snippet above
// - the main function (fn main) is always the first code that runs in every executeable program
// - use the use statement to bring crates (name for packages in rust) into the scope of the program you 
// wish to use them in
// - fn main is the starting/entry point into every rust program
// - println stands for print line and is a macro that is why it is prepended by a bang !
// - let keywords create a variable in rust
// - by default all variables in rust are immutable, use mut to make them mutable
//   let apples = 5; // immutable
//   let mut bananas = 5; // mutable
// :: as use with the String::new() lets us call a function associated with a type (i assume this is synomynous with a method in python)
// the 2 lines after the io:stdin() could have been written on a single line and the code would still work fine
// but long lines of codes are usually difficult to read, hence breaking it into multiple lines makes it easier to read
// you can use the curly brace as a placeholder in printing a value
// let x = 3;
// let y = 5;
// println!("x = {x}") OUTPUT: x = 3
// println!("y + 2 = {}", y + 2) OUTPUT y + 2 = 7
// a crate is a collection of rust source code == packages in python

// it should be noted that the symver way of noting version indicates the series to work with
// examples a.b.c means atleast a.b must anything more than a.b should be ignored

// because cargo.lock file is needed for  reproducible builds
// it is often checked into version control system to be keep tracked off along side the source code