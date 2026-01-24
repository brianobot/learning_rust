fn main() {
    // if statement is as expected and as it functions in other programming languages
    // the if blocks are themselves expressions as you see the do not end in a semicolon
    let condition = true;

    // if it also important to note that the condtion to be use in an if block must evaluate to a bool
    // unlike languages like ruby, and javascript, rust will not try to convert non-boolean to boolean
    if condition == true {
        // blocks of codes associated with an if statements are sometimes called arms
        // like the arms in the match statment we saw in the game
        println!("Well Well Well, Isn't today Amazing");
    } else {
        println!("Weird! Today is Weird");
    }

    // let try something, lets make the condition an expression
    // the code below did not work, i f*cked around and found out for you, you are welcome
    // if { true } {
    //     println!("It workss");
    // } else {
    //     println!("it does not work");
    // }

    // multiple else if blocks can be uses
    let number = 5;

    if number == 0 {
        println!("Number is Zero");
    } else if number == 1 {
        println!("Number is One");
    } else if number == 2 {
        println!("Number is Two");
    } else if number == 3 {
        println!("Number is three");
    } else {
        println!("Number is not Zero, One or Two");
    }

    // just like in most programming languages only one arm is tried at most,
    // so even if subsequent arm's condition evaluate to true, only the first arm
    // that evaluate to true would be processed

    // if can be used in a let statement as shown below
    let condition = true;
    let answer = if condition { 5 } else { 0 }; // remember number are themselves expressions
    // when using the if else arm to assign value to a variable
    // both expression must evaluate to the same data type,
    // trying to assign different datatypes would lead to the compiler producing an error

    println!("The value of answer is {answer}");

    // LOOPS:
    // rust has three kind of loops
    // while, for and loop

    // loop runs forever until it is explicitly stops
    let limit = 5;
    let mut count = 0;
    loop {
        println!("Count = {count}");

        if count >= limit {
            break;
        }

        count = count + 1;

        // continue is used to skip the remaining part of the loop and go to the
        // next interation
    }

    // when breaking out of a loop, you can pass a value after the break statment to be received by the caller of the loop
    let mut counter = 0;
    let result = loop {
        println!("Counter = {counter}");

        if counter == 10 {
            break counter * 2;
        }

        counter += 1;
    }; // notice how this loop has a semi-colon, because it is a statment becuase of the let infront

    println!("The result is {result}");

    // loops label
    // when using nested loops, it can be very useful to apply break or continue to a particular less of nesting
    // by referring to the loop label

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    // Condition loops with while
    // while loops are similar to the syntax used in python
    let mut is_active = true;
    let mut counter = 1;
    while is_active {
        println!("Loop is active ... {counter}");

        if counter % 5 == 0 {
            is_active = false
        }

        counter += 1;
    }

    // For loop, exactly like python only that is uses curly braces for it block
    let arr = [10, 20, 30, 40, 50];

    // using for loops eliminate bugs that might arise from go outside the bounds of
    // a collection (example an array, )
    for element in arr {
        println!("Element: {element}");
    }

    for element in (1..=10).rev() {
        println!("Count Down: {element}");
    }
    println!("Lift Off...");

    loop {
        println!("Some information..");
        break;
    }

    let mut outer_count = 0;
    // loops can be label too with the 'label syntax
    'outer: loop {
        outer_count += 1;
        println!("Running outter loop");
        let mut inner_count = 0;
        'inner: loop {
            inner_count += 1;
            println!("Running inner loop");

            if inner_count > 54 {
                println!("Ended Inner Loop");
                break 'inner;
            }

            if outer_count > 54 {
                println!("About to end outer loop");
                break 'outer;
            }
        }
    }

    let number = 10;
    println!("Number: {}", number);
    // assert_eq!(number, 10);
    // assert_eq!(number, 12);
}
