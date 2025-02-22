fn main() {
    // the result type is used to represent an outcome of a computation that may fail
    // it is an enum with two variants Ok and Err

    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            return Err(String::from("Division by Zero is not allowed"));
        }
        Ok(a / b) 
    }

    let result_1 = divide(1.2, 3.2);
    let result_2 = divide(3.4, 0.0);

    for item in [result_1, result_2] {
        match item {
            Ok(value) => println!("Value = {}", value),
            Err(_) => println!("Encountered error while processing computation"),
        }
    }
    
}