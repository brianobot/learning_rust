// tests are just regular functions with the #[test] attributes
// in the body of the test functions, we use certain macros to assert the state
// we are testing

fn sum(_a: i8, _b: i8) -> i8 {
    4
}

fn guess_year() -> i32 {
    2024
}

#[test]
fn test_sum_function() {
    let result = sum(1, 2);
    assert_eq!(result, 4);
}

#[test]
fn test_guess_year_function() {
    let year = guess_year();
    assert_eq!(year, 2024);
}