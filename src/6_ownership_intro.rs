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
    
}