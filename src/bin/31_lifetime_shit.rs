/*
There are 2 types of lifetimes
- Concrete lifetimes
- Generic lifetimes

Concrete lifetimes refer to the span of code for which a value is valid
Generic lifetimes are used to annotate relationship between lifetimes, there are needed when working wit references
since it is diffcult for the compiler to concretely tell the lifetime of those referenced values
*/

fn main() {
    // example of concrete lifetime
    let value = String::from("Some Important Information"); // value comes to life here

    println!("{}", value); // value is still alive at this point

    drop(value); // this simulates moving the value out of scope
    // at this point the value is dropped and it lifetime ends... adios value

    // this is a valid use of value
    // println!("{}", value);

    let s1 = String::from("Brian");
    let s2 = String::from("David");

    let longest = get_longest(&s1, &s2);
    println!("{}", longest);
}

fn get_longest(a: &str, b: &str) -> &str {
    if a.len() > b.len() { a } else { b }
}