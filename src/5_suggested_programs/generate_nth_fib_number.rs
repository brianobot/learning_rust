use std::io;

fn main() {
    println!("Generate the Nth Fibonacci Number");
    println!("===================================");

    let mut index = String::new();

    println!("Enter the Nth Position you want to find: ");

    io::stdin()
        .read_line(&mut index)
        .expect("Cound not read from the input");

    let index: i32 = index.trim().parse().expect("Could not convert into Integer");

    // Fn = Fn-1 + Fn-2
    // 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144

    let result = fib(index);

    println!("The Number at the {index}th Position is {result}");

}

fn fib(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n-1) + fib(n-2) // this is recursion going on here
    }
}
