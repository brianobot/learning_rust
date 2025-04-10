#![allow(unreachable_code)]

fn main() {
    // diverging functions are functions that never return,
    // there are annoated with an exclamation for their return type, this is an empty type

    void();

    // because they never return, they can never be instantiated, this is different from the unit type
    // that has exactly one type

    println!("This part of the code would never be reached!");
    let _x = panic!("Other Universe");
}

fn void() -> ! {
    panic!("This is the edge of the universe");
}
