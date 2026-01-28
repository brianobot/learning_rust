
use std::io::Write;


#[allow(dead_code)]
// this can be read as a mutable reference to any value that implements the Write trait
fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}


fn main() -> std::io::Result<()> {
    // defined a function that takes in any value that implements the Write trait
    use std::fs::File;
    let mut local_file = File::create("hello.txt")?;
    say_hello(&mut local_file)?; // works
    
    let mut bytes = vec![];
    say_hello(&mut bytes)?; // also works
    assert_eq!(bytes, b"hello world\n");
    
    
    /// Given two values, pick whichever one is less.
    /// whenever a specification is given for the type of a generic,
    /// it is called a bound because it specifies a bound for the type needed to be used for the value
    fn min<T: Ord>(value1: T, value2: T) -> T {
        if value1 <= value2 {
            value1
        } else {
            value2  
        }
    }
    
    // generics and traits are closely related
    // generics used trait in bounds to specify what type of arguments can be applied to them
    // 
    let mut data = vec![1, 2, 3, 4];
    data.write(&[5, 6, 7]).unwrap();
    
    println!("Data = {:?}", data);
    // when using a trait method, the trait itself must be in scope 
    // otherwise the methods would be hidden
    
    
    std::io::Result::Ok(())
}