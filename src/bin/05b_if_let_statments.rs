#![allow(dead_code, unused_variables)]

fn main() {
    // destructuring is when we place a parenthesis on the left side of assignment operator "="
    let (a, b) = (1, 2);

    // use more the dbg marco to view the state of variables
    // instead of printing formatted strings of the variables
    dbg!(a, b);

    struct Thing(u32);

    // this is also possible in cases like this
    let Thing(c) = Thing(5);
    dbg!(c);

    enum Season {
        Winter,
        Autumn,
        Spring,
        Summer,
    }

    use Season::*;

    let random_season = Autumn;
    let random_season = Spring;
    let random_season = Summer;
    let random_season = Winter;

    match random_season {
        Winter => println!("It is the Winter ❄️"),
        Autumn => println!("It is the Autumn 🍂"),
        Spring => println!("It is the Spring 🍃"),
        Summer => println!("It is the Summer ☀️"),
    }

    // see how we use the if let Structure to match against and the the variable
    // we are matching on the right side of the assignment operator
    if let Winter = random_season {
        print!("Snow Snow and Lots of Snow");
    }
    // it is important to note that this expression above is not a statement
    // so it does not have to be terminated with a semi-colon,
    // the return value of said expression can be assigned to a variable

    let party_count = if let Summer = random_season {
        println!("Party Party Party and lots of party");
        3
    // notice how we have to include an else branch when returning from the it let expression
    } else {
        0
    };

    dbg!(party_count);
}
