use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    // you can make your own types support arithmetic and other operators 
    // by implementing a few built in traits, this process is called operator overloading
    
    // Operations    Operator
    // std::ops::Neg    -x
    // std::ops::Not    !x
    // 
    
    Ok(())
}