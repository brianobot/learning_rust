#![allow(dead_code, unused_variables)]

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
    #[allow(dead_code)]
    struct Ascii(Vec<u8>); // using this is much better than passing around Vec<u8> and explaining the intent with commends
    
    // a unit like struct is a type with only one value
    #[derive(Debug)]
    struct Onesuch;
    
    let o = Onesuch;
    println!("O = {:?}", o);
    
    struct Queue {
        older: Vec<char>,
        younger: Vec<char>,
    }
    
    impl Queue {
        fn push(&mut self, c: char) {
            self.younger.push(c);
        }
        
        fn pop(&mut self) -> Option<char> {
            if self.older.is_empty() {
                if self.younger.is_empty() {
                    return None;
                }
                
                use std::mem::swap;
                swap(&mut self.older, &mut self.younger);
                self.older.reverse();
            }
            
            self.older.pop()
        }
        
        fn get(self: &mut Queue, c: char) -> bool {
            self.older.contains(&c)
        }
    }
    
    // functions defined in an impl block are called associated functions, the opposite of this is the regular functions called
    // free functions because they are no associated with a type
    // 
    // when a method is called, rust passed the value the method is called on as the first argument to the method
    // this is called self, since self type is obviously the one at the top of the impl block
    // rust let's you ignore the type
    
    // fn push(self: &mut Queue, c: char) { ... } // this is the same thing as our push method above
    let mut queue = Queue { older: vec!['a', 'b', 'c'], younger: vec!['1'; 10]};
    
    let contains_2 = queue.get('2');
    println!("Contains 2: {contains_2}");
    
    // when calling the method, rust handles implicitly borrowing references or mutable references to 
    // the type the method is called on
    // 
    // queue.get("") -> (&mut queue).get(...)
    // 
    // a method self argument can also be a Box<Self>, Rc<Self> or Arc<Self>
    // such a method can only be called on a value of the given pointer type
    // calling the method passes ownership to the pointer 
    // 
    // you do not usually have to do this tho, since normal referenced self like  can be when called on
    // any of these pointer types 
    let result = Queue::get(&mut queue, '2');
    println!("Result = {result}");
    
    // you can have multiple different blocks for impl for a type
    // but they must all be in the same crate as the type
    // Everything in rust is a value
    struct GQueue<T> {
        older: Vec<T>,
        younger: Vec<T>,
    }
    
    // this might feel redundant, but the impl<T> shows that the imply block is for any type of T
    // as opposed to something like impl GQueue<u64>
    // 
    // as a shorthand every impl block defines the special tyome parameter Self
    impl<T> GQueue<T> {
        fn new() -> Self {
            Self { older: Vec::new(), younger: Vec::new() }
        }
        
        fn push(&mut self, value: T) {
            self.older.push(value)
        }
    }
    
    
    let mut char_queue = GQueue::new();
    char_queue.push('c');
}