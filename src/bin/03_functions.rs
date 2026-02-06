fn main() {
    // the fn allows for the creation of new named functions
    // the main function is the starting point for every rust program

    // rust uses snake case for function names just like python
    println!("Hello, World!");

    // rust does not care where the functions are defined
    // provided there are in the scope of the place they are called from.

    // Rust regular functions can not take a variable number of arguments
    // Rust macros can, which is why the println! is implemented as a macro

    another_function();
    function_with_arg(5);

    let five = with_return_value();
    println!("Five = {five}");

    let value = with_return_keyword();
    println!("Value = {value}");
}

fn another_function() {
    println!("ANother function ... ");
}

// when defining functions with arguments the type of the arguments must be provided
// syntax arg: type, ...
fn function_with_arg(x: u32) {
    println!("X = {x}");
}

// Rust is an expression-based language
// - Statements are instructions that perform some action and do not return a value.
// - Expressions evaluate to a resultant value. Let’s look at some examples.
// let y = 10;  // statement
// functions are also statements

// you can not do this in rust
// let x = (let y = 10); // which is equivalent to x = y = 6 which works in other languages

// expressions evalute to a value
// let x = 6; // 6 is an expression to evaluate to 6
// calling a function is an expression, calling a macro is an expression
// a new scope block created with curly braces is an expression
// expression do not end with semicolon, adding semicolon to it turns it into a statement
// which then makes it not to return anything

// to return a value from a function, you must specify the type of the return value and the last
// expression the function block would be returned, you can force an early return by using the word return
// but in most rust programs, the implicit expression return is employed

// Since passing arrays as function argument is quite limiting
// since the array type (element type and length) would limit the arrays
// the function can be applied to, we can instead pass slices to functions
// that expect sequence , and the arrays and vectors would be treated alike

fn with_return_value() -> u32 {
    5
}

fn with_return_keyword() -> f64 {
    64.0
}
