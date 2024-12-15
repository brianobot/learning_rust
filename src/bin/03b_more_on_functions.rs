fn apply<F>(f: F) where F: FnOnce() {
    f()
}

fn main() {
    // remember there is not restriction on the order of the function definitino and call in rust
    // _fizzbuzz_to(100);

    let x = 10;

    // closures are functions that can capture the enclosing environment
    // and trust me, i tried doing so with the regular function and it did not compile
    let speaker = || { println!("Calling Speaker: {x}") };
    speaker();

    // closures use || instead of () for input variables
    // closures do not require the {} to delimit their body for single line expression

    // closures can capture variables in 3 ways
    // - by reference &T annotated as Fn
    // - by mutable reference &mut T, annotated as FnMut
    // - by value T, annotated as FnOnce

    // despite the annotation on the closure, the compiler would choose the method of capture that is less restrictive

    // they prefer to capture reference and only go lower when needed
    // the by value captures usually occurs for type that are non_copy
    // to force closure to capture a variable by value, use the keyword move before the pipes

    let haystack = vec![1, 2, 3, 4];
    let contains = move |needle: i32| haystack.contains(&needle);

    println!("Contains 9: {}", contains(9));
    println!("Contains 1: {}", contains(1));
    
    apply(speaker);

    // if a capture variable is dropped within the closure, this forces the compiler to capture 
    // by value, since the closure is taking charge of the capture variable

    // it should be noted, that normal functions can be used in certain places where closures are required
    // if the statisfy the type bound and trait bound of the higher order function

    // closures can be returned a returned values, but they MUST be annotated with the impl Trait to return them
    // in this case, the closure must use the move keyword to take ownership of the capture variables to ensure
    // there is no dangling reference if is does capture any enclosing variable that is

    fn create_fn()  -> impl Fn() {
        let _text = String::from("New");
        move || println!("Creating New Closure for: {}", _text)
    }

    let small_fn = create_fn();
    small_fn();

}


fn _fizzbuzz_to(range: u32) {
    for i in 1..=range {
        if i % 15 == 0 {
            println!("Fizzbuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

// Associated functions and methods
// Associated functinos are accessed with the :: (double colon) 
// while methods are accessed with the . (dot notation)

struct _Box {
    width: u8,
    heigth: u8,
}

impl _Box {
    fn _new() -> Self {
        _Box { width: 0, heigth: 0 }
    }

    // &self is usually used in place of the full form self: &Self, where Self point to the type the function is 
    // defined on, self gives access to the type fields via dot notation
    fn _area(self: &Self) -> u8 {
        self.width * self.heigth
    }

    // the full form of the type is self: &mut Self
    fn _transmute(&mut self) {

    }

    // types with self, consume the type they are called on
    fn _destroy(self) {

    }

    // mutable objects can call mutable methods

}