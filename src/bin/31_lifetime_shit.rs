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

// by default the gives an issues that says `missing lifetime specifier`
// to fix this, a generic lifetime must be added to the function signature to help
// the compiler understand the expected lifetime for each of the references used in the function
fn get_longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    // this basically means the lifetime 'a is expected to be equal to the
    // SHORTEST lifetime for the input parameter
    if a.len() > b.len() { a } else { b }
}
