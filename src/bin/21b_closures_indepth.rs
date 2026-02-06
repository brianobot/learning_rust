#![allow(clippy::redundant_closure_call)]

fn main() {
    let y = 12u8;
    let adder = |x| x + y;
    let result = adder(12);
    println!("Result = {}", result);

    let f = add; // f is function pointer to the function add
    let result_2 = f(1, 4);
    println!("Result 2 = {}", result_2);

    // inline closure defintion and call
    let value = |x: i32| -> i32 { x + 1 }(1);
    println!("Value = {value}");

    // when annotated function pointers are represented with fn
    // when closures are denoted with Fn*

    // we can use closures in places where a function pointer type is expected so long as the closure does not capture any environment
    // variable, so closures that do no capture environment variables are similar to function pointers
    // functions pointers and closures that do not capture environment are interchangeable

    // Fn is a trait and is different from the fn (function type)
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
