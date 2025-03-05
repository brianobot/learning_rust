// In Concurrent programming, the program does multiple things at the same time (or atleast appears to)
// Threading is one form of concurrency, this is handle by the operating system
// in Async, the concurrency happens entirely within the program, the Os is not involved
// An Async Runtime manages async tasks by working with the programmner who explicity yield control
// by using the await keyword

// In order for async to work, the tokio crate must be installed in the current project 
// cargo add tokio --features=full

async fn say_hello() {
    println!("hello, world!"); // this is a bad idea to do blocking operations in async function but we move
}

#[tokio::main] // this makes my main function an async function which is otherwise not permitted in rust
async fn main() {
    say_hello().await; // an async function does nothing unless it is awaited with the await 
}

// async is an annotation on functions and some other items such as traits
// await is an operator used in expressions

// An async runtime is needed to manage the operation of async tasks and to manage async io operations
// with the OS

// The #[tokio::main] annotation initializes the Tokio runtime and starts an async task for running the code in main.
// Later  in this guide we'll explain in more detail what that annotation is doing and how to use async code without it 
// (which will give you more flexibility).

// the basic unit of async currency is future, a future is a regular rust objects that implements the Future trait
// a future refers to a defered computation, a computation that would be ready at some point in the future
// futures can be combined to make bigger futures

// A task is a Future which is executed

// An async function is a function annotated with the keyword async, which means the function would be executed concurrently
// which means the caller can choose not to wait for the function to complete before doing something else

// when an async function is executed, the body and it;s argument are packaged into a future which is returned in place
// of a real result , the caller can then choose what to do with the future, if they want the result straight away
// they call the await

// within the async function code is executed in the usual sequential fashion, you can call synchronous function from
// async functions 

// futures in rust are an inert object, in order for them to do work they must be riven forward by an external force
// usually an async runtime

// async can onl be used inside an async context
// you can imagine the run time as a global varaible that is only accessible within in async function

