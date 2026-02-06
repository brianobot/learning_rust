#![allow(clippy::nonminimal_bool)]

fn main() {
    // integers 1
    // floats 1.23
    // boolean true
    // char
    // unit types () can be written as literals

    // integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);

    // short circuiting boolean values
    println!("TRUE and FALSE = {}", true && false);
    println!("True OR False  = {}", true || false);
    println!("Not True = {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
