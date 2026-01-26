

fn main() {
    // A grouping a associated types that are treated as a unit
    // they are 3 types of structs
    // named field
    // tuple like and 
    // unit like
    // 
    struct Grayscalemap {
        pixels: Vec<u8>,
        size: (usize, usize)
    }
    
    // constructing a value of this type
    let (width, height) = (1024, 576);
    let image = Grayscalemap {
        pixels: vec![0; width * height],
        size: (width, height)
    };
    
    // there's a short hand for populating field with local variables if the names of the local variable match the field names
    fn new_image (size: (usize, usize), pixels: Vec<u8>) -> Grayscalemap {
        Grayscalemap { pixels, size } // notice how we only provided the field name here, without the colon
    }
    
    // to create a struct using struct expressions all it fields must be visible
    // this is why we can't create String and Vector with struct expression, even though they are just structs
    // 
    // when using struct expressions, if named fields are by ..EXPR, 
    // then any fields not mentioned take their values from EXPR, which can be another struct
    // 
    // defining the type also defines a function that returns the tuple struct when called
    // this also tuple struct to be used in context where a function is expected like the map iterator method
    // 
    // we can also use the tuple struct to wrap another type and createa newn type like shown below
}