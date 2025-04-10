#![allow(dead_code, unused_variables)]

fn plus_one(a: i32) -> i32 {
    a + 1
}

fn main() {
    let two = plus_one(1);
    let one = plus_one(0);

    // this is a function pointer to the plus_one function
    // without type declaration
    let fn_ptr = plus_one;
    println!("three = {}", fn_ptr(2));

    // with type declaration
    let fn_ptr2: fn(i32) -> i32 = plus_one;
    println!("four = {}", fn_ptr2(3));
}
