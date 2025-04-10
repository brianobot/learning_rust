fn main() {}

/*
This functions takes in any type that implements the as_ref trait
*/
pub fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

pub fn strlen2<S>(s: S) -> usize
where
    S: AsRef<str>,
{
    s.as_ref().len()
}

pub fn foo() {
    strlen("Hello wordl"); // this takes in a &str
    strlen(String::from("Hello there")); // this takes in a String

    // the generic functions is flexible, but behind the scene this is possible due to monomorphization
    // the compiler generates a function code for all types that match the trait defined in the function
    // signature, so there is a strlen(s: &str) and a strlen(s: String) etc

    // Monomorphization is the process of generating concrete implementation for each type that matches a trait
    // NOTE: it does not generate for all possible types but for those that are demanded in the code,
    // that means for type that match the trait but weren't demanded in the code as at compile time, the concrete types are not
    // generated.
    // this process does not just happen for functions, it happens for types too
    // One downsize of this is that binary get duplicated for each concrete types
}

pub trait Hei {
    fn hei(&self);
}

impl Hei for &str {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

pub fn boo() {
    "J".hei();
}

pub fn bar(h: &str) {
    // this is static dispatch before the compiler knows exactly what method call here
    h.hei();
}

// Impl when used in function defintion is syntatic sugar for generic
// both boo1 and boo2 are equivalent to each other
pub fn boo1<H: Hei>(s: H) {
    s.hei();
}

pub fn boo2(s: impl Hei) {
    // this is a shortcut for the first signature style
    s.hei();
}

// the limitation with the simple generic type usage in function is that
// per function, a concrete type is actually used at the end of the day,
// so if we needed to access an array of different types that implemented a particular
// trait, this wouldn't work within the context of a single function

pub fn zoo(_s: /*  &[dyn Hei] */ impl Hei) { // the idea here is to get a slice of types that implements the Hei trait
                                             // we do not care about their concrete types, just that they implement Hei, now
                                             // since this can be different types, generating a single type instance for each type needed
                                             // would not cut it, since within the slice, the first item might be a different type from the rest
                                             // for h in s {
                                             //     h.hei();
                                             // }
}

// to fix the issue about, we have to put the dyn <Trait> behind some kind of pointer time
// since all pointer types are sized, we can ensure dynamic dispatch without monomorphization
pub fn zoo_fixed(s: &[&dyn Hei]) {
    for h in s {
        h.hei();
    }
}

// trait objects are objects that only has the property that it represent a trait
// a trait object is a reference to something that implements a particular trait
// the following are trait objects
// - &dyn Hei
// - Box<dyn Hei>
//
// it is essentially type erasure

// unlike static dispatch where the compiler can tell the data type passed in the place of the generic
// in the functions calls and actually swap them out for their concrete type during compilation,
// the input for trait objects are not knowable at compile time and are only known at compile time

// now the trait object &dyn <Trait> is a fat/wide pointer because it holds two pointers
// fats pointers are references that act like pointers but hold additional information about the
// thing they are pointing to

#[allow(unused)]
pub trait Animal {
    fn noise(&self);
}

struct Dog {}

struct Antelope {}

impl Animal for Dog {
    fn noise(&self) {
        println!("bark!");
    }
}

impl Animal for Antelope {
    fn noise(&self) {
        println!("Nigh");
    }
}

pub fn get_dog(_a: u8) -> impl Animal {
    Dog {}
}

pub fn get_antelope(_a: u8) -> impl Animal {
    Antelope {}
}

pub fn get_random_animal(a: u8) -> Box<dyn Animal> {
    if a > 5 {
        Box::new(get_dog(a))
    } else {
        Box::new(get_antelope(a))
    }
}
