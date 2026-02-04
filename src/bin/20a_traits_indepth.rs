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
    // trait objects can be created by using a reference or some sort of fat pointer 
    // followed by the keyword dyn and the name of the trait 
    // 
    // example: &dyn Write -> this is a trait object to the Write Trait
    //          Box<dyn Read> -> This is a trait object to the Read trait
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
    trait Vegetable {}

    #[allow(dead_code)]
    struct Salad<V: Vegetable> {
        veggies: Vec<V>,
    }

    // in the same above, all Salad in the Salad would be of the same type

    #[allow(dead_code)]
    struct Salad2 {
        veggies: Vec<Box<dyn Vegetable>>,
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
    // you can't implement trait you don't own on types you didn;t defined
    // this is called the orphan rule and it help rust ensure each trait implementation is unique
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

    // SUbtraits allow use to declare that some trait are an extension of another trait
    enum Direction {
        Forward,
        Backeward,
        Upward,
        Right,
        Left,
        Null,
    }

    trait Creature: Visible {
        fn position(&self) -> (i32, i32);
        fn facing(&self) -> Direction;
    }

    // this means that every type that implement creature must also implement visible
    // creature is a subtrait of visible and visible is a supertrait of creature
    // technically a subtrait is a trait which a bound on another trait
    // trait Creature where Self: Visible {
    //     ...
    // }
    //
    // fully qualified method call can be used in place of regular method calls
    // it's important to know that methods call are just a special kind of function call where the value
    // it is called on is passed as the first argument to the method

    "hello".to_string();
    ToString::to_string("hello");
    str::to_string("hello"); // all of these are equivalent 

    // strangely enough it can be beneficial to use qualified method call in some cases
    // case 1: clashing method names from different traits
    // case 2: unknown type when calling the method

    let zero = 0;
    let result = i64::abs(zero);

    println!("Result: {result:?}");

    // traits that show relationship
    // the key skill is the section below is the ability to read traits and method signature
    // and figuure out what they say about the types involved

    pub trait SampleIterator {
        type Item; // associated type

        fn next(&mut self) -> Option<Self::Item>;
    }

    // generic code can use associated type
    fn collect_to_vector<T: Iterator>(iter: T) -> Vec<T::Item> {
        let mut vector = Vec::new();
        for value in iter {
            vector.push(value)
        }
        vector
    }

    let result = collect_to_vector(vec![1, 2, 3, 4, 5].into_iter());
    println!("Vector: {result:?}");

    fn dump<I>(iter: I)
    where
        I: Iterator,
        I::Item: Debug,
    {
        for (index, value) in iter.enumerate() {
            println!("{index}: {value:?}");
        }
    }

    // remember that concept of subtrait, we can write subtraits like this
    // Iterator<Item=String>, this is basically Iterator whose produced values are String
    // this can be used anywhere a regular trait is used

    fn str_dump<I: Iterator<Item = String>>(iter: I) {
        for (index, value) in iter.enumerate() {
            println!("{index}: {value:?}");
        }
    }

    fn dyn_str_dump(iter: &mut dyn Iterator<Item = String>) {
        for (index, value) in iter.enumerate() {
            println!("{index}: {value:?}");
        }
    }

    str_dump(vec!["".to_string(); 100].into_iter());

    // generics traits
    trait SpecialMul<RHS = Self> {
        // the RHS means that SpecialMul is a genereic trait that defaults to the type of the
        // implementator if a type is not provided
        type Output;

        fn special_mul(&self, rhs: &RHS) -> Self::Output;
    }

    // in rust, the expression
    // lhs * rhs is shorthand for Mul::mul(lhs, rhs)
    // so overloading the * operator is as simple as implementing the Mul trait
    //
    // type erasure can be achieved using a impl trait type declaration
    // this basically means any type that implements this trait without worrying about dynamic dispatch like &dyn Trait (trait object) does

    
    trait GenericStuffs<T> {
        fn talk(&self) -> T;
    }
    
    struct MyType;
    
    
    impl GenericStuffs<u32> for MyType {
        fn talk(&self) -> u32 {
            u32::default()
        }
    }
    
    impl GenericStuffs<String> for MyType {
        fn talk(&self) -> String {
            String::default()
        }
    }
    
    let my_type = MyType;
    let value: u32 = my_type.talk();
    println!("Value = {value}");
    
    fn f(a: &dyn Debug, b: &dyn Debug) {
        println!("a = {a:?}, b = {b:?}");
    }
    
    f(&10, &10.2);
    
    fn g(a: Box<dyn Debug>, b: Box<dyn Debug>) {
        println!("a = {a:?}, b = {b:?}");
    }
    
    g(Box::new(10), Box::new(13.4));
    
    
    std::io::Result::Ok(())
}
