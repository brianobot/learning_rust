/*
Tests are Rust functions that verify that non-test code is functioning as expected.

Test usually perform these 3 actions
- Set up any needed data or state
- Run the Code you want to test
- Assert That the result are what you expect

*/

// at it simplest,  a test is a function that's annotated with the test attribute

fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[test]
fn test_add() {
    assert!(add(2, 2) == 4);
}
