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
    // 
    // the ownership rule only applies at compile time to ensure that the program is valid
    // 
    // just as variables owns their values
    // struct owns their fields, and vectors, arrays and tuples owns their elements too
    #[derive(Debug)]
    struct Person {
        name: Option<String>,
        age: u32
    }
    
    // the composer variable owns the Vec<Person>
    // the Vec<Person> Owns each person inside of it
    // Each person Struct owns the name and the age field
    // Each Name field owns the buffer stored on the heap for the String
    let mut composers = Vec::new();
    composers.push(Person { name: Some(String::from("J.Cole")), age: 45 });
    // composers.push(Person { name: Some(String::from("Drake")), age: 45 });
    // composers.push(Person { name: Some(String::from("A$AP Rocky")), age: 45 });
    // composers.push(Person { name: Some(String::from("Post Malone")), age: 45 });
    // composers.push(Person { name: Some(String::from("Fela Kuti")), age: 45 });
    
    // dbg!(composers);
    
    // More on Moves
    // assigning a value to a variable
    // passing a value to a function
    // returning a value from a function 
    // all these actions move a value, the source relinquishes ownership of the value
    // and become uninitialized, the desination now becomes the owner of the value and manages it lifetime
    let s = vec!["ramen".to_string(), "noddles".to_string(), "cake".to_string()];
    println!("S: {s:?}");
    
    let t = s;
    println!("T: {t:?}");
    
    let u = t;
    println!("U: {u:?}");
    
    // building a tuple moves values into the tuple
    // moving a variable in a loop is bad, since after the first iteration the variable would be uninitialized
    // 
    // let x = vec![1, 2, 3];
    // while true {
    //   f(x)
    // }
    //  this can only work if we keep giving it a new value at the end of each iteration
    // 
    // Notes: a moves always leaves the source uninitialized
    // Rust does not allow for partial moves of Vector items
    // 
    // let v = vec![1, 2, 3, 4];
    // 
    // let third = v[2]; this would lead to an error since rust can not move out the third element and have a partially initialized vector
    // when we pass a vector a loop, it is moved into the loop
    // 
    let first_name = composers[0].name.take();
    println!("First Name: {first_name:?}");
    
    dbg!(composers);
    
    // As a Rule of Thumb any types that needs to do some thing special when it's dropped should be a copy
    // a vector needs to drop it elements, a file type needs to close it file handler
    // By default struct and enums are not Copy
    // you can make your own struct Copy by deriving Copy and Clone Trait for them
    // provided all it's field implement the Copy trait
    // Notice how when implementing Copy, Clone is also implemented, this is intentional
    #[derive(Debug, Copy, Clone)]
    struct Label {
        number: u32
    }
    
    let label_1 = Label { number: 1 };
    let label_2 = label_1;
    
    println!("Label 1: {label_1:?}");
    println!("Label 2: {label_2:?}");
    
    // using a copy trait on a type indicates that the implementor wishes to commit to none heap allocated field
    // 
}
    
