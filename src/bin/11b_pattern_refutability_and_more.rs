
fn main() {
    // patterns comes in two form,
    // - refutable: pattern than can fail to match for some possible values
    // - irrefutable: pattern that always match

    let _x = 5; // is irrefutable, cause it always match
    let some_value = Some(1);
    if let Some(x) = some_value { println!("X = {x}")} // this is refutable cause, the pattern is not match when some_value is None

    // let, for loops, can only accept irrefutable patterns

    // patterns can be used to destructure structs too
    struct Point {
        x: i8,
        y: i8,
    }

    let p = Point { x: 1, y: 2 };

    let Point {x: a, y: b} = p;

    println!("A = {a}");
    println!("B = {b}");

    //  ignoring all or parts of a pattern 
    // we can use the _ to ignore part of a pattern

    fn _foo(_: i32, y: i32) {
        println!("Y = {y}");
    }
    // in the function above, only the second parameter is used, the first is ignored
    // this can be particularly useful when implementating traits that require a particular type signature
    // but we do not use those paramters in the body of the functions/methods

    // we can ignore an unused variable by starting it with underscore
    let _unused_variable = 123;

    // we can ignore remaining parts of a value with ..
    let data = (1, 2, 5, 5, 3, 6, 2, 7, 4, 3);

    match data {
        (first, .., last) => { println!("First: {first}, last: {last}")},
        // _ => println!("Strange Case"),
    }

    // we can use extra conditions with match guards to control the matches arms that are executed
    let some_data = Some(26);

    match some_data {
        Some(n) if n % 11 == 0 => { println!("{n} is Divisible by 11") },
        Some(n) if n % 13 == 0 => { println!("{n} is Divisible by 13") },
        Some(n) if n % 17 == 0 => { println!("{n} is Divisible by 17") },
        Some(n) if n % 19 == 0 => { println!("{n} is Divisible by 19") },
        _ => println!("Other case")
    }

    // the downside with using match guards, is that the compiler does not try to check for exhiastiveness
    

    // create an infinite iterator
    // let inf = (0..).collect::<Vec<u128>>();

    // for i in inf {
    //     println!("I: {}", i);
    // }

    // @ sigil binding in pattern expression
    // the @ sigil is particular important when working with range values
    let age = 23u32;

    match age {
        n @ 0..=5 => println!("{n} Stil an Infant"),
        n @ 6..=11 => println!("{n} Now  a Child"),
        n @ 12..=19 => println!("{n} Now a Teenager"),
        n @ 20.. => println!("{n}! Proper Adult and all"),
    }

    let number = Some(7);

    // if let can also be used in the same way as below to match an Enum
    if let Some(a) = number {
        println!("A = {a}");
    }

}