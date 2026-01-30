#![allow(dead_code)]
use std::io::{Error, Write};


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
    #[allow(dead_code)]
    fn min<T: Ord>(value1: T, value2: T) -> T {
        if value1 <= value2 { value1 } else { value2 }
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
    //
    // we can rewrite our say hello plain function which took a trait object as an argument as a generic function as so
    fn say_hello<W: Write>(out: &mut W) -> Result<(), Error> {
        out.write_all(b"hello world\n")?;
        out.flush()
    }

    // notice that the body of the function is still the same
    // now when you call this function, rust generate machine for the specific type used to call this function
    // so if you call this function for 3 different types, rust would generate machine code for those three types
    // this process is known as monomorphization,
    //
    // you can always spell out the type parameter using turbo fish
    let mut local_file = File::create("hello.txt")?;
    say_hello::<File>(&mut local_file)?;

    // if it important to always specify the type for a generic function that takes no argument
    // an example of this is the collect<C>() method on iterators
    //
    let v1: Vec<i32> = (0..=1000).collect();
    println!("V1: {:?}", v1);

    use std::fmt::Debug;

    // when declaring a generic function you can specify the abilities need from the generic parameter by adding trait bounds
    fn top_three<T: Debug>(values: &Vec<T>) {
        let line_len = 30;
        println!("{}", "-".repeat(line_len));
        println!("First Three Items: ");
        println!("{}", "-".repeat(line_len));
        for i in 0..3 {
            println!("   value = {:?}", values[i]);
        }
        println!("{}", "-".repeat(line_len));
    }

    top_three::<f64>(&vec![1., 2., 3., 4., 5., 0.6]);

    // generic functions can have multiple type parameters
    #[allow(dead_code)]
    fn run_query<T, M, X, P>(_data: &T, _input: &M, (_x, _y): (X, P)) {}

    // if the bounds reqiured for the parameters are long
    // rust provides another way to express the bounds
    //
    #[allow(dead_code)]
    fn run_query_2<T, M, X, P>(_data: &T, _input: &mut M, (_x, _y): (X, P)) -> ()
    where
        T: Write,
        M: Debug,
    {
        ()
    }
    
    // this type of bounds is permitted any where bounds are permitted
    // when declaring a generic function that have lifetimes, the lifetimes comes first
    // 
    // an individual methof can be generic even if the type if's defnined on is not generic
    
    // type aliases can be generic too
    // 
    // Which to use: Generic functions or functions that take trait objects
    // use trait objects when you need you need a collection of values of mixed types
    // 
    trait Vegetable {
        
    }
    
    #[allow(dead_code)]
    struct Salad<V: Vegetable> {
        veggies: Vec<V>
    }
    
    // in the same above, all Salad in the Salad would be of the same type
    
    #[allow(dead_code)]
    struct Salad2 {
        veggies: Vec<Box<dyn Vegetable>>
    }
    
    // this version can contain any veggie as far as they match the trait object declared here
    // each Box<dyn Vegetable> can contain any value, but the items in the Vec are all the same
    // Box, which is a fat pointer to some type, in this case, to a trait object type
    
    // another reason to use trait object is too save binary size
    // because generic would generate a a version of the funciton for each type it is used for
    // 
    // advantanges of generic over trait objects
    // 1. generic are faster and can be optimized since rust knowns what function or trait methods to use
    // 2. not all trait can supoprt trait objects
    // 3. generic can specify multiple bounds on multiple trait, but trait object can onlt be used with one trait except with subtrait
    
    // Defining Trait
    // when defining a trait you specify the signature of the trait methods and you can provide default implementation
    // by adding a body to those method signature
    // when implementing a trait, you can only specify things that were listed in the trait declartion
    // if you need other methods, define them in your value impl block, those values would be visible to the trait methods
    
    #[allow(dead_code)]
    trait Visible {
        fn draw(&self);
        fn hit_test(&self, point: (i32, i32)) -> bool;
    }
    
    #[allow(dead_code)]
    struct Broom {
        pos: (i32, i32),
    }
    
    #[allow(dead_code)]
    impl Broom {
        fn outline(&self) -> (i32, i32) {
            self.pos
        }
    }
    
    impl Visible for Broom {
        fn draw(&self) {
            // notice how the method of the value is available here
            let _bound = self.outline();
            // draw the bounds
        }

        fn hit_test(&self, (x, y): (i32, i32)) -> bool {
            let bound = self.outline();
            x == bound.0 && y == bound.1
        }
    }
    
    // when implementing a trait for a value, you can skip the default method
    // and only implement those without a default implementation
    // 
    // you can implement any trait on any tupe
    // as long as either the trait or the type is introduced in the current crate
    // 
    trait IsEmoji {
        fn is_emoji(&self) -> bool;
    }
    
    impl IsEmoji for char {
        fn is_emoji(&self) -> bool {
            *self == '😂'
        }
    }
    
    assert_eq!('😂'.is_emoji(), true);
    
    
    // Notes: Trait methods are only visible when the trait is in scope
    // An extension trait is a trait used for an existing type like char, str etc
    // 
    // you can define a generic trait like so
    
    struct HtmlDocument(String);
    
    trait WriteHtml {
        fn write_html(&mut self, html: &HtmlDocument) -> std::io::Result<()>;
    }
    
    impl<W: Write> WriteHtml for W {
        fn write_html(&mut self, _html: &HtmlDocument) -> std::io::Result<()> {
            std::io::Result::Ok(())
        }
    }
    
    // The line impl<W: Write> WriteHtml for W means “for
    // every type W that implements Write, here’s an
    // implementation of WriteHtml for W.”
    
    std::io::Result::Ok(())
}
