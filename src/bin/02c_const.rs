#![allow(dead_code)]

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
}