
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
    // the reason Clone and Iterator work is because they are included in the rust standard prelude
    
    // dyn Write is known as a trait object
    // 
    // Rust doesn't permit variables of type dyn Write
    let mut buf: Vec<u8> = vec![];
    // let writer: dyn Write = buf; // this does not work, because the size of write must be known
    // but a value implementing Write could be any size
    let _writer: &mut dyn Write = &mut buf; // a reference to a trait type is called a trait object
    
    // Rust will automatically converts types that implements a references of types that implement a certain trait
    // to their trait object when they are used in places that require a trait object
    // 
    // imagine a type 
    // let file = File::new(...), let assume File implements a Write trait
    // if you pass file to a function that requires a &mut dyn Write
    // &mut file is converted to the trait object &mut dyn Write
    // 
    // this conversion is neccesary because for rust to be able to call a method on a trait object
    // it needs access to the vtable for that trait object, so when you pass a reference to a concrete that
    // rust takes that and adds the address of the appropriate vtable to a new created trait object so it can call the correcrt method
    
    std::io::Result::Ok(())
}