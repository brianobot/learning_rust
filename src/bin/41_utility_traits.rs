#![allow(dead_code, unused_variables)]

// you can think of utility trait as a bunch of traits in the standrd library that has alot of impact on the way Rust is wrriten
// mastering them would make one write more idiomatic code and build crates public interfaces better

// broading speaking, there are categorized into following sets
// - Language extension traits: like the operator overloading traits, Drop traits, Deref, DerefMut and conversion trait
// - Marker traits: traits used to expression contraits on generic types that can be caputured in any other trivial way, Size and Copy
// - Public Vocabulary Trait: they are not magival to the compiler but using them mirrors convetional solutions for common problems, Default, AsRed, AsMut, Borrow and BorrowMut

use std::fmt::Debug;

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

    let mut names = vec![1, 2, 3, 4, 5];
    println!("Capacity: {}", names.capacity());
    println!("Length: {}", names.len());

    names.truncate(4);
    println!("After Truncation");
    println!("Capacity: {}", names.capacity());
    println!("Length: {}", names.len());

    #[derive(Debug)]
    struct Person {
        name: String,
    }

    impl Drop for Person {
        fn drop(&mut self) {
            println!("Dropping Person: Name = {}", self.name);
            // because the drop method on a value is called, before the value is actually drop
            // the value is still fully initialized and all it's field can still be used
        }
    }

    let brian = Person {
        name: "Brian".to_string(),
    };
    println!("✅ Start of Section on Drop Trait");
    println!("brian: {brian:?}");
    println!("person name: {:?}", brian.name);
    // rust calls the drop trait of a type if it implement the Drop trait
    // and then after that drops the value, so the value is not dropped in the drop
    // method of the Drop trait but some other logic can happen there
    //
    {
        let mut _person = Person {
            name: "Albert".to_string(),
        };
        println!("Before Assignment");

        _person = Person {
            name: "James".to_string(),
        };
        println!("At the end of the block");
    }

    println!("✅ End of Section on Drop Trait");

    // if a variable value is moved into another variable, when the variable goes out of scope
    // rust will not try to drop it, since it does not contain any value
    // if a type implement Drop it can not implement Copy and vice versa

    // Sized: is a type whose values have the same size in memory
    // all sized types implement the Sized marker trait which has no methods or associated types
    // you can not implement it yourself, the only need of the Size trait is as a trait bound for types that implement it
    //
    // rust can not store unsized values in variables or pass them around
    // that why you must interact with them through pointers, like & and Box
    // a pointer to a unsize value is always a fat pointer, a pointer to the item in the head and the length
    let information = "Some Information that might be important for the next learner";

    // all generics types are implicitly Sized
    // when you write fn gen<T>() {} rust treats T as fn gen<T: Size>() {}
    //
    // Clone: Clone must be infalliable
    // Copy: these are the exception to the moving values rule of rust, since the types that implement this
    // are simple and do not have resources, the are simply Copied, a Copy is an subtrait of the Clone trait

    trait Copy: Clone {} // it is a marker trait
    // types that own resources like heap buffers or operating system handles can not implement copy
    // likewise any type that implement the Drop trait can not implement Copy
    //
    // Deref and DerefMut: this specifies how Rust handles the dereferencing operators like * and .
    // let b = Box<Complex> implements Deref and DerefMut, so *b refers to Complex, and b.re refers to the real part of Complex

    #[allow(dead_code)]
    trait Deref {
        type Target: ?Sized;

        fn deref(&self) -> &Self::Target;
    }

    #[allow(dead_code)]
    trait DerefMut: Deref {
        fn deref_mut(&mut self) -> &mut Self::Target;
    }

    // Target should be something that Self Contains, owns or refer to
    let underlying_value = 5;
    let door = &underlying_value;
    let opened_door = *door;

    assert_eq!(5, underlying_value);
    assert_eq!(5, opened_door);

    struct MyBox<T>(T);

    impl<T> std::ops::Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let my_box = MyBox(10);
    assert_eq!(10, *my_box); // *(my_box.deref())

    #[derive(Debug)]
    struct Selector<T> {
        elements: Vec<T>,
        current: usize,
    }

    impl<T> std::ops::Deref for Selector<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.elements[self.current]
        }
    }

    impl<T> std::ops::DerefMut for Selector<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.elements[self.current]
        }
    }

    #[allow(unused_mut)]
    let mut s: Selector<i32> = Selector {
        elements: vec![1, 2, 3, 4],
        current: 2,
    };

    assert_eq!(*s, 3);
    // assert!(12i32.is_positive());
    assert!(s.is_positive());

    // you can spell out coercion using the as keyword in rust
    fn spell(number: &i32) {
        println!("Spelling: {number}");
    }

    // since the spell function takes a reference to i32, we can coerce s to that
    spell(&s as &i32);
    #[allow(clippy::explicit_auto_deref)]
    spell(&*s); //both are valid

    // Default: the default trait allows types to specify their default via a default method
    impl<T: Default> Default for Selector<T> {
        fn default() -> Self {
            Self {
                elements: Vec::default(),
                current: usize::default(),
            }
        }
    }

    let default_selector = Selector::<i32>::default();
    println!("Default Selector: {default_selector:?}");

    // all of rust collection types implement default that return empty collections
    let samples = vec![2, 3, 45, 6, 23, 23, 5, 6, 34, 2, 4, 6];
    let (x_men, impure): (Vec<i32>, Vec<i32>) = samples.iter().partition(|&n| n & 1 == 0);

    println!("X-Men: {x_men:?}");
    println!("Impure: {impure:?}");

    // if a type implements Default, then the standard library implements defaults for Arc<T>, Rc<T>, Box<T> etc

    // when a type implements AsRef<T>, it means you can borrow &T from it efficiently
    // this can support some nice functionality by taking a generic type that implements a particular AsRef<> and
    // then calling the as_ref method of that type to use the reference you actually want

    // for example the the std library fs open method has this signature
    // std::fs::File::open;
    // pub fn open<P>(path: P) -> io::Result<File>
    // where
    // P: AsRef<Path>,
    //
    // in the body of the function, you really just want to get the reference to Path, so no matter what is passed in
    // calling as_ref on it returns the reference to path, but this the function to allow different types
    // making it look like an operator overloaded function
    //
    // any types U that implements the AsRef<T> get an automatic implementation for it's &U also implementing AsRef<T>
    // this serves as a conversion trait, you should avoid defining a trait for conversion like as AsFoo, when you
    // use the AsRef<Foo> trait to get a reference to the Foo type for any type

    // Unlike AsRef<T> and AsRefMut<T>, the as From<T> and Into<T> take ownership of the implementor and returns a new type
    /*
     * trait From<T>: Sized {
     *    fn from(other, T) -> Self;
     * }
     *
     * trait Into<T>: Sized {
     *    fn into(self) -> T;
     * }
     *
     * the compiler autoimplementes the conversion of every type to itself
     * for example every T implements a From<T> and an Into<T>
     */

    // you generally use into to make function arguments more flexible in the options they accept
    /*
     * use std::net::Ipv4Addr;
     *      fn ping<A>(address: A) -> std::io::Result<bool>
     *     where A: Into<Ipv4Addr>
     * {
     *      let ipv4_address = address.into();
     *     ...
     * }
     *
     *
     */
    // we have TryFrom and TryInto as equivalent versions of From and Into which can fail and therefore return Result<T, Error>
}
