// Try is used to propagate error
// Result is the main process for handling error, Result reprsent a state of success
// of failure, and the ? operator can be applied on any expression that returns a Result, also the ? operator
// can only be used on Result instance inside a function that returns a Result type

fn add() -> Result<(), ()> {
    Ok(())
}

fn calculate() -> Result<(), ()> {
    let value = add()?; // at this point,
    // if the add function (expression) is an error, the function ends here and is returned
    // notice how the return value here matches the expected function signature
    Ok(value)
}

fn main() {
    let value = calculate().unwrap();
    println!("Value = {:?}", value);
}
