fn main() {
    // generic for functions intro by Brian Obot

    // in order to reduce code duplication, we can define and use type parameters
    // to represent abstract data types in our function definition, this type parameters
    // are usually single Letter , e.g T for T but if it is a word, the camelcase are usually followed
    // the type parameters are used as the usual function parameters are used, but before using a type 
    // parameter, it must be defined for the compiler to know what it is


    fn largest<T:Ord+Eq>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let some_integers = [1, 2, 4, 3, 2, 5, 2];
    let largest_integer = largest(&some_integers);
    println!("Largest Integer = {largest_integer}");

    // the code block below would not work, because the floating point types
    // do not implement the Ord or Eq trait which are needed to be allowed as a type parameter T

    // let some_floats = [2.3, 52.3, 52.3, 32.4, 43.1];
    // let largest_float = largest(&some_floats);
    // println!("Largest float = {largest_float}");

    /*
    We read this definition as: the function largest is generic over some type T. This function has one parameter named list, which is a slice of values of type T. The largest function will return a reference to a value of the same type T.
     */


    // generic for struct

    // we can also use a generic type parameter in one or more fields using the <T> syntax to define the type in the struct 
    // definition

    // i am using a module here to avoid name collision of the Point struct and to be able
    // to keep both definition with the same name in the same source file
    mod first {
        #[derive(Debug)]
        pub struct Point<T> {
            pub x: T,
            pub y: T,
        }

        // the definition allows x and y to be any type provided they are same as each other

    }

    let would_work = first::Point { x: 5, y: 3};
    // let wont_work = Point { x: 4.3, y: 2}; 
    println!("Would work = {:?}", would_work);
        
    mod second {
        // now to define a struct with fields that can be same types but can also be different
        // define different types for those fields

        #[derive(Debug)]
        pub struct Point<T, U> {
            pub x: T,
            pub y: U,
        }
    }

    /*
    Now all the instances of Point shown are allowed! You can use as many generic type parameters in a definition as you want, but using more than a few makes your code hard to read. If you're finding you need lots of generic types in your code, it could indicate that your code needs restructuring into smaller pieces.
     */

    let both_integer = second::Point { x: 5, y: 4};
    let both_float = second::Point { x: 1.0, y: 2.0};
    let integer_and_float = second::Point { x: 1, y: 3.4};

    println!("Both integer = {:?}", both_integer);
    println!("Both Float = {:?}", both_float);
    println!("Integer and float = {:?}", integer_and_float);

    // generics in enum definition

    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }


}