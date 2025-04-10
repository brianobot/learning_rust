/*
Rust offers the ability to write unsafe codes


things that can be done in unsafe rust
- deference a raw pointer
- call and unsaf function or method
- access or modify static mutable variable
- implement unsafe trait
- access fields of union


NOTE: unsafe does not turn off the borrow checker 
*/
#![allow(unused)]


static HELLO_WORD: &str = "Hello, World!";
static mut COUNTER: u32 = 0;


fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 5;

    // 1.
    // this is an immutable raw pointer that points to a i32 value
    // the asterisk is not not 
    let r1 = &num as *const i32;
    // this is a mutable raw pointer that points to a i32 value
    let r2 = &mut num as *mut i32;

    // raw pointers do not follow rust borrow checker rules
    // they are not guarantee to point a valid value

    // rust allows us to create raw pointers in the context of sage rust
    // but in order to deference the raw pointers,that must be done within an unsafe block

    unsafe {
        // notice how we created a mutable and an immutable raw pointer to the same address
        // in regular references in rust, this is not possible
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // 2.
    // calling unsafe functions or methods
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    // unsafe functions must be called within unsafe blocks
    // or within other unsafe functions

    // implementating safe abstraction
    // we can have safe functions that have unsafe implementations in them but can be called and used as regular
    // safe functions


    // Extern functions to call external code
    extern "C" {
        fn abs(input: i32) -> i32;
    }


    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // 3. Access and Modify mutable stable variable
    add_to_counter(3);
    unsafe {
        #![allow(static_mut_refs)]
        println!("COUNTER: {}", COUNTER);
    }

    // 4. Implementation of Unsafe trait
    // a trait is unsafe when atleast one of it method is unsafe

    unsafe trait Foo {}

    unsafe impl Foo for i32 {}

    // Union
    union Win {
        bronze: i32,
        silver: i32,
        gold: i32,
    }



    let win = Win { silver: 30 };

    unsafe {
        println!("WIn field: {:?}", win.silver);
    }
}