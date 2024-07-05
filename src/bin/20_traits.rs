#![allow(dead_code)]

fn main() {
    /* Traits in Rust are a way to define shared behavior across multiple struct or enum types. They're similar to  interfaces in other languages. A trait is defined with the keyword 'trait' followed by its name and a set of method signatures within curly braces.
    */

    // the trait only defines the methods needed (the interface)
    trait LivingThing {
        fn breathe(&self);
        fn die(&self);
        fn love(&self);
        fn can_code(&self) -> bool;

        // it is possible to define default implementations for the traits function
        // and this defaults can be used by the types that implement the trait or it can be 
        // overriden, 

        fn summarize(&self) -> String {
            let can_code = &self.can_code();
            let formatted_string = format!("I a living thing and i can code == {can_code}");
            String::from(formatted_string)
        }
    }


    struct Dog;

    // the struct or enum that implements the traits must then implement
    // the body of the trait functions/methods

    impl LivingThing for Dog {
        fn breathe(&self) {
            println!("I am breathing");
        }

        fn die(&self) {
            println!("Nice One, But i am not dead yet");
        }

        fn love(&self) {
            println!("Yayyyyy! I can do this");
        }

        fn can_code(&self) -> bool {
            false
        }
    }

    let dog = Dog;
    dog.breathe();
    dog.die();
    dog.love();
    let message = dog.summarize();
    println!("{}", message);

    //Traits as Parameters
    // we can use the traits to define functions that expects types that implements those traits

    fn introduce_self(item: &impl LivingThing) {
        let message = item.summarize();
        println!("🔥 {}", message);
    }

    introduce_self(&dog);

    // the method define above is a syntactic sugar for a longer form of syntax called trait bound syntax
    fn introduce_self_v1<T: LivingThing>(item: &T) {
        let message = item.summarize();
        println!("🔥 {}", message);
    }

    introduce_self_v1(&dog);

    // we can return a type that implements a certain traits
    fn return_living_thing() -> impl LivingThing {
        Dog
    }

    let _a_living_thing = return_living_thing();



}