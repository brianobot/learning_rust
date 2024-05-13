// #[allow(dead_code)] // this disables the dead code warning on compile

fn main() {
    // printing is handled by a series of macros defined in std::fmt 

    /*
        format! : write formatted text to String
        print!: same as format! but the text is printed to the console (io::stdout).
        println!: same as print! but a newline is appended.
        eprint! : same as print! but the text is printed to the standard error (io::stderr).
        eprintln!: same as eprint! but a newline is appended.
     */

    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // just like in python formatted string, positional arguments can be specified
    println!("{2}:{1}:{0}", 23, 59, 7);

    // names arguments can also be used a statement can be spread over multiple lines
    // provided the semi-colon is not used, a st
    println!(
        "{producer} told {consumer} that EVERYTHING will be alright! and {producer} believed it",
        producer="Brian",
        consumer="himself", // notice that the trailing comma did not cause any errors, just like in python
    );

    // just like in old boy python, different format specifiers can be used to format a value
    println!("++++++++++++++++++++++++++++++++++++");
    println!("Base 10: {}", 69420); // 69420
    println!("Base 2 (binary): {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal): {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // text can be aligned to some side with the specifier shown below
    println!("Number: {number:>5}", number=1);
    // you can pad the spacing 0 (or any other single character) as shown
    // when you specify more than 1 
    println!("Number: {number:0>5}", number=2);
    println!("Number: {number:*>5}", number=3);
    println!("Number: {number:9>5}", number=4);

    // you can use name specifier by appending them with $
    println!("Special Number: {number:+>width$}", number=12, width=10);
    // arguments from surrounding variables can be captured in the format specifier
    let number = 10;
    let width = 5;
    println!("Captured Number: {number:+>width$}");

    // only types that implement the fmt::Display can be used in format string
    // user defined types do not support fmt::Display by default
    /* struct Structure(i32);

    let structure = Structure(3);
    println!("structure: {}", structure);
    */

    // `Structure` cannot be formatted with the default formatter

    // Activities
    let pi = 3.141592;
    // this is quite intuitive coming from the same functionality in python
    println!("PI = {pi:.2}");

    // probably not the best place to place this information
    // but macros are not called like functions but are expanded into source code that get
    // compiled with the rest of the codes, this is a very important distinction between functions
    // and macros
    
}