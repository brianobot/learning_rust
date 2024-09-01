fn main() {
    // tuples can hold multiple values 
    let tup: (i32, i32, i32, i32, i32, i32) = (1, 2, 3, 4, 5, 6);
    let state = (1, 4);

    let reverse_state = reverse_pair(state);

    println!("Tup = {:?}", tup);
    println!("Reversed state = {:?}", reverse_state);
}


fn reverse_pair(pair: (i32, i32)) -> (i32, i32) {
    let (first, second) = pair;

    (second, first)
}