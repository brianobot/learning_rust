fn main() {
    // the most primitive datatype in rust is the str
    // and it is known as string slice

    let x: i32 = 42;
    let crab: String = String::from("🦀");

    println!("Number of 🦀 = {x} {crab}");

    // maximum value of i32 = 2 ^ 31 - 1
    // maximum value of i64 = 2 ^ 63 - 1

    // maximum value of u32 = 2 ^ 32
    // maximum value of u64 = 2 ^ 64

    let is_snowing = false;
    println!("Is it Snowing? {is_snowing}");

    // char type
    let x = 'x';
    println!("X = {x}");
    
    // Rust is very strict about the use of bool expression
    // unlike in python where empty datatypes can be used as false bool indications
    // where rust expect a bool, a bool expression must be used
    // instead of if X { ... } 
    // if x.is_empty() { ... } or if x == 0 { ... }
    // 
    // booleans can be converted to integers types with the as operator
    let is_old_enough = true;
    let int_form = is_old_enough as i32;
    println!("Int Form: {int_form}");
    
    // Rust represents a single Unicode character as a 32-Bit value
    
}
