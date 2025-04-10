#![allow(dead_code)]
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl From<i8> for Number {
    fn from(item: i8) -> Self {
        Number { value: item as i32 }
    }
}

// into is the reciprocal of the from trait
// and when you implement the from trait you get the into for free
// when using the into, you must specify the type to convert into
// ass the compiler usualy does not know this automatically

fn main() {
    // the From trait provide implementations for data types to create itself from other types

    let num = Number::from(32);
    let another_num = Number::from(32i8);

    let integer: Number = 32i8.into();

    println!("My number is {:?}", num);
    println!("Another Number is {:?}", another_num);

    println!("Integer = {:?}", integer);

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    println!("Parsed = {}", parsed);
    println!("Turbo parsed = {}", turbo_parsed);
}
