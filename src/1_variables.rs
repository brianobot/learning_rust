fn main() {
    // VARIABLES
    let x = 5;

    println!("The value of x is {x}");
    // this causes the compiler to produce an error,
    // cause it is not allowed to assign values more than ones to a variable by default
    // x = 6;
    // println!("The New value of x is {x}");

    // variables can be made mutable by adding the mut keyword infront of them
    let mut y = 6;
    // it is important to know that assigning a variable as mut and not actually mutuatin it
    // causes the compiler to produce an error message, so only use mut if you indeed need to mutate
    // the variable in question
    println!("The value of y is {y}");
    y = y + 10;
    println!("The new value of y is {y}");

    // CONSTANTS are not allowed to be made mutable, they are always immutable
    // with constant the type MUST BE annotated
    // it is convention to use all upper case and understand for constants in rust
    // since the value is evaluated at compile, writing it as an expression makes it 
    // easier to understand instead of putting the final result there

    // constants are valid for the entire time the program run in the scope in which there
    // are defined
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS = {}", THREE_HOURS_IN_SECONDS);

    // SHADOWING
    // the new variables that shadows the former are still immutable
    // this process enables use to perform transformation on variables and
    // ensure that they remain immutable after these transformation.
    let age: u32 = 23;

    {
        let age = 12;
        println!("Inner Scope Age = {age}");
    }

    println!("Outer Scope Age = {age}");
    // another difference between shadowing and mutating is that, we can only mutate the variable
    // value with mut, but with shadow we are technically creating a whole new variable which is not limited
    // to the afore referenced variable type
    
}