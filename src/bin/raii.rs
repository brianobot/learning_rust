fn main() {
    // just messing around to see what breaks
    let x: i32 = 10;
    println!("The initial value of x = {x}");

    // it is nice to observe that the variable x is redefined here as mutable and this works
    // it is also good to notice that if i did not use the first x variable
    // the compiler would throw a warn about unusead variable there
    let mut x: i32 = 100;
    x = x * 12;
    println!("The Final value of x = {x}");

    // so the idea with RAII is that all values (resources) in rust have an owner
    // and when that owner goes out of scope, the resources are released or removed from memory

    // this ensures that one bit of memory only has one owner, this ensure that the system
    // does not try to access invalid memory or bear dangling memory references
}
