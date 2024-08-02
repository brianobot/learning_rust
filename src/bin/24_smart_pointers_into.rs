#![allow(dead_code)]

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // a pointer is a variable that contains an address in memroy
    // that points to some other data. the most common type of pointer in rust 
    // is just a reference, they are denoted with & and they borrow the values they point
    // to
    let value: i8 = 4;
    let reference = &value;
    // allows you to create a reference to a value without making the right side a reference
    let ref another_ref = value;

    println!("Reference = {:?}", reference);
    println!("Deference reference = {:?}", *reference);
    println!("Another reference = {:?}", another_ref);

    match reference {
        val => println!("Value gotten = {}", val),
    }

    match value {
        // we can make a call the ref as so too
        ref v => println!("Reference gotten = {:?}", v),
    }

    // smarts pointers are 
    // 1. usually structs with metadata about the data they point to
    // 2.they own the data they point to 

    // they implement the Deref and Drop traits

    // Box<T>
    // allows to store data on the heap rather than on the stack
    // and the pointer to the heap data is kept on the stack, 
    // it has no performance overhead

    let b = Box::new(5);
    println!("b = {b}");

    // here 5 is the data store on the heap, 
    // and the address to this data is stored on the stack
    // when the b, the Box goes out scope, the pointer is cleared and so is the 
    // data that is stored on the heap

    /*
        stack       | heap
        --------------------
        b <pointer> | 5

     */

    // Box<T> is a pointer

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List = {:?}", list); 



}