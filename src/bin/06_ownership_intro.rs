// RESEARCH ON FILE NOT INCLUDED IN CRATE HEIRACHY WARNING
fn main() {
    // create a varible to hold a string
    let s1 = String::from("hello");

    // NOTICE the user of :? in the placeholder, this is supposed to be a debug print
    // ?? why does the debug version have quote surrounding the string data
    println!("Debug Word: {:?}", s1);
    println!("Word: {s1}");

    // at this point the s1 variable is considered invalid as it data is not owned by s2
    let _s2 = s1;

    // trying to access s1 here will throw an error
    // println!("Trying to access s1: {s1}");

    // what happened here is that, when you assign another variable to the former, 
    // rust moves, the data from the former to the new, making the former invalid, 
    // this way, when those variables go out of scope only the  valid variables heap 
    // data would be cleared, this way no error occurs trying to clear double clear the same memory location

    // now the same logic when used for an integer would not break
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
    /*
        But this code seems to contradict what we just learned: we don’t have a call to clone, but x is still valid and wasn’t moved into y.

        Explanation as shown in the docs:
        The reason is that types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make. That means there’s no reason we would want to prevent x from being valid after we create the variable y. In other words, there’s no difference between deep and shallow copying here, so calling clone wouldn’t do anything different from the usual shallow copying, and we can leave it out.
     */

    // ownership and function
    /*
    the same rules mentioned above apply for move data into functions
    when a variable is passed to a function, it is moved to that function, 
    like the string are move, and not accessible afterwards in the scope it was defined, but for types, like integer, since the there are entirely alive on the stack, they can still be accessed, since a simple copy == deep copy for them
     */


    // so basically we have 4 actions 
    // move, copy, borrow and borrow mutably
    // move - data stored in heap are moved (string)
    // copy - data stored in stack (numbers, boolean)
    // borrow - pass a reference to the data, that way, the data can be used after being borrowed (it must not be mutated while in borrowship tho) 
    // borrow mutably

    // Three Rules for Rust Ownership
    // - Each value has a variable that is called it owner
    // - the can only be one owner at a time
    // - when the owner goes out of scope, the value will be dropped

    // special case for arrays
    // if the elements of an array implement the copy trait, then the array also implements the copy trait
    // when we pass types that implement the copy trait to a function, they are no moved, but copied,
    // and the others still keep ownership of them
    
}