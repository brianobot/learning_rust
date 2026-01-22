fn main() {
    // a closure is usually consisted of an arugment list given between vertical bars followed by an expression
    let is_even = |x| -> bool { x & 1 == 0 };

    let result = is_even(2);
    println!("Is Even: {result}");
}
