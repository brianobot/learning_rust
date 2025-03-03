fn main() {

}


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

pub fn bar(h: &str) { // this is static dispatch before the compiler knows exactly what method call here
    h.hei();
} 

// Impl when used in function defintion is syntatic sugar for generic
// both boo1 and boo2 are equivalent to each other
pub fn boo1<H: Hei>(s: H) { 
    s.hei();
} 

pub fn boo2(s: impl Hei) { // this is a shortcut for the first signature style
    s.hei();
}

// the limitation with the simple generic type usage in function is that 
// per function, a concrete type is actually used at the end of the day, 
// so if we needed to access an array of different types that implemented a particular
// trait, this wouldn't work within the context of a single function

pub fn zoo(s: &[dyn Hei]) { // the idea here is to get a slice of types that implements the Hei trait
    // we do not care about their concrete types, just that they implement Hei, now
    // since this can be different types, generating a single type instance for each type needed
    // would not cut it, since within the slice, the first item might be a different type from the rest
    for h in s {
        h.hei();
    }
}