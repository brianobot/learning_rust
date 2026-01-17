

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
    // 
    // you can initialize an array with a short hand like so
    let zeros = [0; 100];
    println!("Zeros: {zeros:?}");
    
    let mut sieve = [true; 1000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 1000 {
                sieve[j] = false;
                j += 1;
            }
        }
    }
    
    dbg!(sieve);
    
    // you would use similar syntax for fixed size buffer used in codes
    let _fixed_size_buffer = [0u8; 1024];
    
     // the helpful methods you might need for iterating, sorting etc are all provided on slices and not arrays
     // but rust implicity converts a reference to an array to a slice when searching for methods
     let mut chaos = [1, 2, 5, 5, 2, 4, 6, 3, 6, 7];
     chaos.sort();
     println!("Sorted Chaos: {chaos:?}");
}