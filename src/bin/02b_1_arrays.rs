

fn main() {
    // [T; N] represent an array of N values each of type T
    // an array size is constant and is part of it's type
    let _scores = [1, 2, 3, 4];
    
    // &[T] and &mut [T] - shared and mutable slice of Ts
    // these are references to series of values that are part of some other value
    let every_one_but_first = &_scores[1..]; 
    println!("Everyone Score But the First Score: {:?}", every_one_but_first);
    
    // all sequence here, array, vector and slices have a len method
    // and their values can be indexed with sequence[i] where i must be a usize value
    // if the value is out of range rust will panics
}