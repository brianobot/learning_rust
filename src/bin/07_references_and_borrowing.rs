fn main() {
    // a reference is like a pointer, it is an address in which we can follow to access that data being reference
    // this referenced value is owned by another variable and unlike a pointer, every reference guarantees that
    // it is pointing to a valid value of a particular type for it lifetime

    let s1 = String::from("Hello");

    let len = calculate_len(&s1); // the ampersand represents references
                                  // they allow you to refer to some value without taking ownership

    // the action of creating a reference is called BORROWING!

    println!("s1 = {s1} len = {len}");

    let mut s2 = String::from("World");
    // mutable references have one big restriction, once you have one mutable reference to a value
    // you can not have any other references to that value

    change(&mut s2);

    show(&s2); // this reference here is getting the updated value of the value, SMH

    /*
       Users of an immutable reference don’t expect the value to suddenly change out from under them! However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.
    */
    
    let _some = &String::from("Brian"); // this is pronouced ref String
    // this is a reference of type String
    // think of a reference to a i32 as a single machine word holding the address of the i32
    // we say a reference to the value is borrowed, let r = &32;
    // to get the actual value, we dereference using *r
    // a reference does not automatically free any resource when it goes out of scope
    // but rust references are never null, there is no way to produce a null reference in safe rust
}

// remember the return type of the function must always be annotated in the function definition
fn calculate_len(s: &String) -> usize {
    println!("Borrowed Value = {}", s);
    s.len()
    // references are immutable by default just like variables
    // so an attempt to change the variable here would cause the compiler to throw an error

    // because s does not own the value it is referencing, when it goes out of scope here
    // that value is not dropped from memory
}

fn show(s: &String) {
    println!("VAlue: {s}");
}

fn change(s: &mut String) {
    s.push_str(" From Africa");
    println!("New Value: {s}");
}
