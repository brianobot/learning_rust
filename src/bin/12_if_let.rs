#![allow(clippy::single_match)]

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let config_max = Some(3u8);
    // here destructing is used to extract and bind the value to max
    // if let allows for a condition to check for and destructing to be done
    // at the same time, so the if let is very closely related to the match
    // expression we met early, but we the if let, we are only
    // checking for specific arms of the possiblities unlike the match expression
    // which is exhaustive
    let max_value = if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
        max
    } else {
        println!("The max value was not configured, defaulting to 10...");
        10
    };

    println!("Max value: {max_value}");
    // we can include an else block with an if let
    // this else block is equivalent to the _ (match all) arm of the match expression

    // both of the codes above are equivalent,
    // if let, let us express cases where we only need to work with the Some Variant
    // of an enum and do nothing with the None variant

    // destructing is important for if let statements
    // some examples of destructing are shown below
    let (a, b) = (10, 20);
    struct Thing(u32);
    let Thing(c) = Thing(21);
    dbg!(a, b, c);
}
