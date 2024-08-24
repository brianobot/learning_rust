
fn main() {
    // rust has two groups of errors
    // recoverable and unrecoverable errors

    // rust uses Result for recoverable errors and panic! macro the unrecoverables ones which must stop the program

    // panic! macro
    // two ways to invoke the panic macro
    // execute a piece of code that leads to a bug like accessing outside the bounds of an array
    // directly invoke panic by calling the panic macro

    // by default panic will show an error message, unwind, clean up the stack and quit
    // if however you do not want rust to handle the unwinding on panic, you can instruct
    // the compile to just panic and abort the program, this leaves the unwinding for the OS
    // [profile]
    // panic = "abort"
    // use this code snippet above in your Cargo.Toml file to activate this no unwinding process

    // panic!("crash and burn");

    let _v = vec![1, 2, 3];

    // v[99];

    // the expect method is a friendly version of the unwrap method
    // it takes a error message that is shown when the program panics
    // let _some_value = [1, 2, 3].iter().nth(100).unwrap();
    let _some_value = [1, 2, 3].iter().nth(100).expect("100th Element not found!");


    
}