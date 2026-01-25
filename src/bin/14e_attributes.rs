

fn main() {
    // every item in rust can be decorated with attributes
    // Attributes are Rust way of writing instructions to the compiler
    // an example is using it to ignore certain compiler warnings
    // 
    #![allow(unused_imports)] // #[allow] attribute
    use std::collections::HashMap;
    
    // some attributes like #[cfg(...)], #[allow(...)] can be applied to a whole module and applies to everything inside of it
    // others like #[test] must be attached to individual items
    // 
    // to attach an attribute to a crate, add it add the top level either at the top of main.rs or lib.rs
    // and use #![] instead of #[], this tells the compiler to add the attribute to the enclosing space rather than the next attribute
    // 
    // remember that tests are ordinary functions marked with the test attribute
    // you can pass argument to tests to control the tests that are ran
    // cargo test math // this run all tests with math in their name
    // assert_eq is just like assert! must if it fails, the error messages shows both value
    // if you want to check certain invariant in debug builds alone, use debug_assert since those are not included in the
    // release build unlike the assert macro
    // 
    // to test error cases, use the should_panic attribute
    // you can also return Result from your test functions
    // 
    // you can include a module only when testing using the #[cfg(test)] attribute
    // when Rust sees comments that starts with three slashes it treats them as a #[doc] attribute instead
    // 
    // these two are equivalent
    /// Simulate the Production of a spore
    #[allow(dead_code)]
    pub fn produce_spore(_factory: u32) -> impl Clone {
        String::from("Spore")
    }
    
    #[allow(dead_code)]
    #[doc = "Simulate the Production of a spore"]
    pub fn produce_sp0re(_factory: u32) -> impl Clone {
        String::from("Spore")
    }
    
    // likewise comments starting with //! are treated as #! and apploed to the enclosing feature
    // usually used at the top of files to add comments to that crate
    // 
    // the contents of a doc comment are interpretted as markdown which is then used to generate html doc
    // you can include html tags and those are used as they are in the generated docs
    
    
}


#[test]
fn test_true() {
    assert_eq!(true, *&true);
}


#[test]
#[allow(unconditional_panic, unused_must_use)]
#[should_panic(expected="divide by zero")]
fn test_should_fail() {
    1 / 0;
}