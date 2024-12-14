fn main() {
    // normally rust can infer types from the values we provide
    // it is important that rust knows the type of all variable at compile time
    // for cases where many types are possible like in the conversion of string to number 
    // as seen in the game with .parse, the type must be annotated

    // unit type
    // represent one datatype that contains nothing, there is only one value
    // for the unit type, it is default return value from assignment expression and functions without
    // an explicit return value, it does not implement the Display trait and so must be printed as a debug value
    // when used in a format string for printing
    let x = ();
    println!("X = {x:?}");

    println!("Some Value = {:?}", some_value);

    let _guess: u32 = "42".parse().expect("Not a number!");
    // note while developing, if you do not use a variable and you want it alive in
    // the code, start it with an undersore and the warning would be ignore when compiling

    // there are broadly 2 data types subset in rust
    // scalar and compounds

    // SCALAR
    // this represent a single value, the usually primitives from most programming languages
    // integer, floating-points, boolean, and characters

    // integers: number with fractional parts
    // options: i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize
    let age: u8 = 24;
    println!("Age = {age}");

    // just like in python, _ can be used as visual seperator to seperate large numbers to make it
    // easeier to read 
    let large_number: i32 = 100_000_000;
    println!("Large number = {large_number}");

    // floating point
    // rust has f32 and f64, the default is f64

    let _x = 2.0; // default to f64
    let _y: f32 = 4.9; // default 32

    // NUMERIC OPERATIONS
    let sum = 5 + 10;
    println!("Sum = {sum}");

    let substraction = 32.42 - 23.45;
    println!("Substraction = {substraction}");

    let quotient = 12.3 / 4.4;
    let truncated = -5 / 3;
    println!("Quotient = {quotient}");
    println!("Truncated = {truncated}");

    let multiply = 2 * 32;
    println!("Multiply = {multiply}");

    let remainder = 4 % 2;
    println!("Remainder = {remainder}");

    // BOOLEAN, true or false
    let active = true;
    println!("Active = {active}");

    let deactivated: bool = false;
    println!("Deactivated = {deactivated}");

    // Character type
    // there are quoted with single quote
    // char is always 4 bytes in size ( 32 bits )
    let _c = 'c';
    let _z: char = 'Z';

    // QUESTION? How much data can a char data type store
    // ANSWER: only character with one codepoint

    let mystr = '✅';
    // also the char datatype can not be empty

    let _heart_eyed_cat = '😻';

    // COMPOUND TYPES
    // compoud types group mutitple values into one, there are two primitives compound types
    // in rust, tuples and arrays

    // Tuples are very much like those in python, once declared, can not change size
    let tup = (2.3, 1, false, true);

    // accessing the tuple values by index using the dot notation
    println!("Tuple at 0 = {}", tup.0); 
    println!("Tuple at 1 = {}", tup.1); 
    println!("Tuple at 2 = {}", tup.2); 
    println!("Tuple at 3 = {}", tup.3); 

    // tuples can be destructured and store their values in variables
    let (x, y, z, w) = tup;
    println!("x = {x}, y = {y}, z = {z}, w = {w}");

    // the tuple without any value has a special name, it is called a unit, 
    // unit = (), it is what is returned by default , if a function does not return a value

    // ARRAYS: fixed length collection of same type elements
    let _a = [1, 2, 3, 4, 5, 6];

    // when annotating an array, we do this
    let _arr: [u32; 5] = [1, 2, 3, 4, 5];
    // the first part of the annotation specifies the type of the array values
    // the second part indicates the length of the array

    // array can also be initialized to contain the same values as follow
    let _same_value_arr = [3; 5];

    // Rust prevents against invalid memory access by panicking and exiting the program

}