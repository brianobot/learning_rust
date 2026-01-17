fn main() {
    // tuples can hold multiple values
    let tup: (i32, i32, i32, i32, i32, i32) = (1, 2, 3, 4, 5, 6);
    let state = (1, 4);

    let reverse_state = reverse_pair(state);

    println!("Tup = {:?}", tup);
    println!("Reversed state = {:?}", reverse_state);
    
    // tuples are an ordered sequence of items which can hold multiple types 
    // tuples only allow constants as indices
    // you can do this
    // let tuple = (1, 2, 3, 4);
    // let tuple.i or tuple[i] this is not correct
    // but this is correct tuple.1, 
    // Rust often use tuples to return multiple types from a function
    // including a trailing comma in tuples is allowed in rust 
    let _valid_tuple = (1, 2, 3,);
    
    // infact Rust does permit a trailing commas any where commas are used
    // function arguments, arrays, structs, enum defintions
    // 
    // for tuple that contain a single value they must be either type annoated as tuples
    // or a trailing comma must be used in their literal form to seperate it from simple parenthetic expression
    let _wrong_single_value = ("Brian"); //
    let _correct_single_value = ("Brian", );
}

fn reverse_pair(pair: (i32, i32)) -> (i32, i32) {
    let (first, second) = pair;

    (second, first)
}
