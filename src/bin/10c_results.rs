fn main() {
    // More explanation for the result Enum
    // Result can be thought of as a generalization of the option enum
    // and the option enum can be thought of as a special case of the result enum

    // enum Result<T, E>{
    //      Ok(T),
    //      Err(E)
    // }

    // enum Option<T>{
    //      Some(T),
    //      None,
    //}

    // Result represent a state of potential error
    // functions that return a Result would usually take ownership of the data
    // they are operating on, this is because they need to return the type
    // in the Ok variant of the Result type as Result<T, E>
}
