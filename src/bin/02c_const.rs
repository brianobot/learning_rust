// #![allow(dead_code)]

// consts are very powerful in rust
const N: usize = 1 + 2;
// constant are immutable
// constant are composable
// constant are evaluated by the compiler
const M: usize = N + 2;

// const can contain logic
const P: usize = {
    let mut n = 1;
    let mut m = 0;

    while m < 10 {
        n += n;
        m += 1;
    }

    n
};

static X: usize = N;

// static is a runtime value and it can be mutable
fn main() {
    let array: [i32; N] = [1, 2, 3];
    println!("array: {:?}", array);

    println!("M: {M}");
    println!("N: {N}");
    println!("P: {P}");

    println!("X: {X}");
}

// constants live for the entire lifetime of the program
// they have no fixed address in memory since they are inlined into the program binary
// statics are global varaible type facilities and they live for the lifetime of the program to
// it is better to use constant than static, since constant does not have a memory address
// the convention for consts and static is to use the SCREAMING_SNAKE_CASE
// usuaully const and statics are place at the top of the code file after module imports
