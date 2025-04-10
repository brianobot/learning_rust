use std::time::Instant;

fn main() {
    let start = Instant::now();
    // the iterator trait provides methods need for iterators in rust
    /*
     * the trait definition looks like
     * pub trait Iterator {
     *   type Item;
     * ...
     * other items left out for simplicity
     * }
     */

    // methods available on the iterator trait include

    // nexts: returns the next item in the sequence
    let a = [1, 2, 3, 4];
    // iterator must be mutable since getting the next items require an internal state change in them
    let mut a_iter = a.iter();

    let next_a = a_iter.next();
    // next returns an Option with the value wrapped if it exists or None it no other value exists
    println!("Next Item in A = {}", next_a.unwrap());
    let next_a = a_iter.next();
    println!("Next Item in A = {}", next_a.unwrap());
    let next_a = a_iter.next();
    println!("Next Item in A = {}", next_a.unwrap());
    let next_a = a_iter.next();
    println!("Next Item in A = {}", next_a.unwrap());
    let next_a = a_iter.next();
    println!("Next Item in A = {}", next_a.unwrap_or(&0));

    println!("Duration = {:?}", Instant::now() - start);
    // depending on the implementation chosen, some iterators may resume returning Some after returning a None

    // next_chunk returns the next N values in a sequence of returns Err containiing an iterator over the remaningin elements
    // this is a nightly feature that i can not use since i am not using a nightly compiler and it could infact be dropped
    // let mut lorem = "lorem".chars();

    // assert_eq!(lorem.next_chunk().unwrap(), ['l', 'o']); // here N is inferred as 2
    // assert_eq!(lorem.next_chunk::<4>().unwrap_err().as_slice(), &[]); // here 4 is explicity provided as N

    // .size_hint: returns a bound the remaining length of the iterator
    let a = [1, 2, 3, 4, 5].iter();

    let size_hint = a.size_hint();
    println!("Size Hint: {:?}", size_hint);
}
