fn main() {
    // Traits, Description
    // Drop: Clean up code rust runs automatically when a value is dropped
    // Size: marker trait for types with a known size at compile time as opposed to dynamically sized types (slices)
    // Clone: type that support cloning
    // Copy: Marker trait for types that can be cloned by moving byte-for-byte copy of memory containing the value
    // Deref, DerefMut: Trait for smart pointers
    // Default: types that have a sensible default value
    // ToOwned: Conversion trait for converting a reference to an owned value
    // From, Into: Conversion trait for converting from one type to another
    // TryFrom, TryInto, Same as above, but might fail
    //

    // Drop
    // you can customize how run process dropping of types by implementing Drop for your type

    #[derive(Debug)]
    struct Person {
        name: String,
    }

    impl Drop for Person {
        fn drop(&mut self) {
            println!("Dropping Person")
        }
    }

    let brian = Person {
        name: "Brian".to_string(),
    };
    println!("brian: {brian:?}");
    println!("person name: {:?}", brian.name);
    // rust calls the drop trait of a type if it implement the Drop trait
    // and then after that drops the value, so the value is not dropped in the drop
    // method of the Drop trait but some other logic can happen there

    // if a variable value is moved into another variable, when the variable goes out of scope
    // rust will not try to drop it, since it does not contain any value
}
