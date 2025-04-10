// in this rust crate, i will take a dive into the world of the rust standard Option Enum type


fn main() {
    // i will explore some if the methods available on the Option enum and how to use them
    // it should be emphasis that the rust prelude, makes the variant of the Option enum to be 
    // directly available in the scope of the crate for use uusing traversing the mod tree
    // using it directly would be importing it like so
    // use std::option::Option;
    let some_value: Option<i32> = Some(12);
    let none_value: Option<i32> = None;

    // is_some method: returns true if the variant is Some
    let is_some = some_value.is_some();
    println!("Some Value is some: {}", is_some);
    let is_some = none_value.is_some();
    println!("None Value is some: {}", is_some);

    // there is a similar is_none method, i would not bother outlining it use here

    // is_some_and method: returns true if the value in the some Variant matches the argument closure value
    let is_some_and_12 = some_value.is_some_and(|x | x == 12);
    println!("Some Value is some and 12: {}", is_some_and_12);
    let is_some_and_4 = none_value.is_some_and(|x| x == 4);
    println!("None Value is some and 4: {}", is_some_and_4);

    // is_none_or: returns true if the variant is None or the value in the predicate is true
    let is_none_or_12 = some_value.is_none_or(|x| x == 12);
    println!("Some value is None or value == 12: {}", is_none_or_12);
    let is_none_or_2 = none_value.is_none_or(|x| x == 2);
    println!("None Value is nonr value == 2: {}", is_none_or_2);

    println!("None value = {:?}", none_value);

    // as_ref converts an &Option<T> to Option<&T>
    let text = Some(String::from("Some String"));
    let as_ref = text.as_ref();
    println!("As Ref = {:?}", as_ref);

    // as_mut converts &mut Option<T> to Option<&mut T>
    let mut text = Some(123);
    let as_mut = text.as_mut();
  
    // basically says only run the block if the pattern is matched 
    if let Some(x) = as_mut {
        *x = 12;
        println!("X = {}", x);
    }

    // .expect: returns the wrapped value in the Some variant or returns the message provided in the call
    let twleve = Some(12);
    let _none: Option<i8> = None;
    let value = twleve.expect("Could not get the value from the Variant");
    // the expect message should tell why you expect the OPtion to be some, e.g Env should not be empty

    println!("Value = {value}");

    // _none.expect("THis would definitely panic");

    // .map: maps an Option<T> to an Option<S> by applying a function to the contain value if some or None if none
    let word = Some("Hello World!");
    let word_length = word.map(|x| x.len());
    println!("Word Length = {:?} contained value = {}", word_length, word_length.unwrap()); 

    // there is map_or that allows to convert from a Option<T> to a type S for Some and a default for None
    let some = Some("foo");
    let none: Option<&str> = None;
    let new_some = some.map_or(5, |x| x.len());
    let new_none = none.map_or(4, |x| x.len());

    println!("New Some = {}", new_some);
    println!("New None = {}", new_none);

    // there is also a map_or_else, does the same thing, but takes two predicate, computers the second predicate for None variant 
    // map_or_else is a special kind of map_or where the default is a predicate (function)
    let new_some_on_else = some.map_or_else(|| "12", |x| x);
    let new_none_on_else = none.map_or_else(|| "12", |x| x);
    println!("New some on else: {}", new_some_on_else);
    println!("New None on else: {}", new_none_on_else);

    // inspect: calls a lfunction with a reference to the contained value if some, returns the original Option
    let some_one = Some(1);
    let none: Option<i8> = None;
    some_one.inspect(|x| { println!("Contained value is {x}") });
    none.inspect(|_x| { println!("No Value is contained in the None Variant") });

    // ok_or , ok_or_else, converts an Option to a Result enum variants, works the same way as map_or and map_or_else
    

}