enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}


fn main() {
    // match allows us to compare a value against a series of patterns and runs 
    // codes based on which pattern was matched

    let dime = Coin::Dime;
    let _penny = Coin::Penny;
    let _nickle = Coin::Nickel;
    let _quarter = Coin::Quarter;

    let value = value_in_cent(dime);

    println!("Coin Value: {}", value);

    // general syntax of the match expression
    /*  this feels very similar to the structure of the if expression
    // but here the value can be any type unlike if which must be boolean
    match value or an expression which evaluaes to a value {
        pattern 1 => code to be ran for pattern 1, // this are called the arm of the match expression
        pattern 2 => code to be ran for pattern 2, // the second part of the match arm is an expression which is the code to be ran
        ...
        pattern n => code to be ran for pattern n,
    }

     */

    // we can bind values from pattern which we can then use in the arm 
    // expression for further processing

    // we can handle Option<T> using the match expression too
    let five = Some(5);
    let zero = Some(0);
    let none = None;

    let six = plus_one(five);
    let one = plus_one(zero);
    let still_none = plus_one(none);

    println!("Six: {:?}", six);
    println!("One: {:?}", one);
    println!("Still None: {:?}", still_none);

    // MAtch are exhuastive, which means all cases must be handled by an arm in the match expression
    // there is a special other pattern that can match all other patterns not listed,
    // above in the other patterns, and this special other arm pattern must always be last,
    // if we do not wish to use the value from the other pattern , we can use the underscore to represent it

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    // this is a reference somewhere in this codebase, but i think it fitting to duplicate it here for context
    // belows are some of the ways to use patterns in rust code
    // - in match expression as seen above
    // - conditional matching with if let

    let some_value = Some(3);
    if let Some(i) = some_value {
        println!("FOund I: {}", i);
    }

    // if let can have an optional else if, and else branchs to handle cases that caught by the if block
    // the downside to using if let pattern is that the compiler does not check for exhaustiveness of the branches
    // and the can be potential logic bugs for cases that are not accounted for completely

    // - similar to the if let conditional pattern above, we have a while let conditional loop
    let mut stack = Vec::<i8>::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.push(5);

    while let Some(value) = stack.pop() {
        println!("📦 Found: {}", value);
    }

    // for loops use patterns too
    // for x in 0..=10 {}
    // the x in the for statement is a pattern and can be more complex than that

    for (index, value) in (0..=10).enumerate() {
        println!(" - {index}: {value}");
    }

    // the let statement we used, to assigned variables are made to use patterns
    // formally the syntax of the let statement is as follows
    // let PATTERN = EXPRESSION

    let range: (i8, i8, i8) = (1, 2, 3);
    println!("Range = {:?}", range);

    // and FUnction parameters are patterns tooo
    // fn some_result(x: i32) -> i32 { x }

    // the x in the function parameter is a pattern and can be more complex

    fn some_result(&(x, y): &(i32, i32)) {
        println!("x: {x}");
        println!("y: {y}");
    }

    some_result(&(23, 45));

    // notice how the function definition come after the function call but the code still works
    fn add_fancy_hat() {
        println!("Adding fancy hat");
    }
    fn remove_fancy_hat() {
        println!("Removing fancy hat");
    }
    fn reroll() {
        println!("Rerolling");
    }
}


fn value_in_cent(coin: Coin) -> i32 {
    match coin {
        Coin::Dime => 5,
        Coin::Penny => 10,
        Coin::Nickel => 20,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}