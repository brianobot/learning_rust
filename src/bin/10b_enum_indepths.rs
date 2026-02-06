#![allow(unused_variables)]
#![allow(clippy::almost_complete_range)]

fn main() -> Result<(), String> {
    // Rust enums are useful when a value might be either one thing or another
    // C-Style Enums
    #[allow(dead_code)]
    enum Ordering {
        Less,
        Equal,
        Greater,
    }

    // the different possible values are called variants or constructors
    // Ordering::Less, Ordering::Equal, Ordering::Greater

    // in memory C-Style enums are stored as integers, you can control the integers to use
    // otherwise rust will assign the integers for you starting from zero
    #[allow(dead_code)]
    enum HttpStatus {
        Ok = 200,
        NotModified = 304,
        NotFound = 404,
    }

    use std::mem::size_of;

    let ordering_size = size_of::<Ordering>();
    let http_status_size = size_of::<HttpStatus>();

    println!("Ordering Enum Size: {ordering_size}");
    println!("Http Status Size: {http_status_size}");

    // rust stores the enums using the smallest built integer type that can accomodate them
    // you can alter this by using the #[repr] attribute
    //
    // you can cast a c style enum into an integer
    // however you can not cast an integer to an enum
    assert_eq!(HttpStatus::Ok as i32, 200);

    #[allow(dead_code)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum TimeUnit {
        Seconds,
        Minutes,
        Hours,
        Days,
        Months,
        Years,
    }

    impl TimeUnit {
        fn plural(&self) -> String {
            match self {
                TimeUnit::Seconds => "seconds".to_string(),
                TimeUnit::Minutes => "minutes".to_string(),
                TimeUnit::Hours => "hours".to_string(),
                TimeUnit::Days => "days".to_string(),
                TimeUnit::Months => "months".to_string(),
                TimeUnit::Years => "years".to_string(),
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    enum RoughTime {
        InThePast(TimeUnit, u32), // this is a tuple variant, like tuple struct, this constructors are functions that create
        // new RoughTime value
        JustNow,
        InTheFuture(TimeUnit, u32),
    }

    impl RoughTime {
        fn verbose(&self) -> String {
            match self {
                RoughTime::InThePast(time_unit, 1) => {
                    format!("{} {} ago", 1, time_unit.plural().trim_end_matches("s"))
                }
                RoughTime::InThePast(time_unit, n) => format!("{} {} ago", n, time_unit.plural()),
                RoughTime::JustNow => "Just now!".to_string(),
                RoughTime::InTheFuture(time_unit, 1) => {
                    format!("{} {} ahead", 1, time_unit.plural().trim_end_matches("s"))
                }
                RoughTime::InTheFuture(time_unit, n) => {
                    format!("{} {} ahead", n, time_unit.plural())
                }
            }
        }
    }

    // enums can also have struct variant which contain name fields like regular structs
    // a single enum can have values of all three variant types unit like, struct variant and tuple variant
    //
    // all constructors in an enum share the same visibility as the enum

    let five_minutes_ago = RoughTime::InThePast(TimeUnit::Minutes, 5);
    let current_time = RoughTime::JustNow;
    let four_minutes_ahead = RoughTime::InTheFuture(TimeUnit::Minutes, 4);
    let one_minute_ago = RoughTime::InThePast(TimeUnit::Minutes, 1);

    println!("{}", five_minutes_ago.verbose());
    println!("{}", current_time.verbose());
    println!("{}", four_minutes_ahead.verbose());
    println!("{}", one_minute_ago.verbose());

    use std::collections::HashMap;

    #[allow(dead_code)]
    #[derive(Debug)]
    enum Json {
        Null,
        Boolean(bool),
        Number(f64),
        String(String),
        Array(Vec<Json>),
        #[allow(clippy::box_collection)]
        Object(Box<HashMap<String, Json>>),
    }

    let null = Json::Null;
    println!("Null Value = {:?}", null);

    // An ordered collection of `T`s.]
    #[allow(dead_code)]
    enum BinaryTree<T> {
        Empty,
        NonEmpty(Box<TreeNode<T>>),
    }
    // A part of a BinaryTree.
    #[allow(dead_code)]
    struct TreeNode<T> {
        element: T,
        left: BinaryTree<T>,
        right: BinaryTree<T>,
    }

    // the only way to access data in an enum is using patterns
    // look at the verbose method of the RoughTime enum to see how patterns are used to accessed data in enums
    // patterns are the parts that appear before the => symbol
    //
    // Expressions produce values
    // Patterns consume values
    //
    // pattern that match a value look like the expression used to produce them
    // when a pattern contains identifiers, those become local variables in the code following the pattern
    // what ever is present in the value is copied or moved into the new local variable
    //
    // Pattern types in pattern matching
    // literals: 100, "name" or name of a const
    // Range: 0..=100, 'a'..='k'
    // wildcard: _
    // local variable: name, mut count
    // ref variable: ref field, ref mut field, borrows a reference to the matched values instead of moving or coopying it
    // binding with subpattern: val @ 0..=99
    // Enum pattern: Some(value), None, Pet::Orca
    // Tuple Pattern: (key, value)
    // array pattenr: [a, b, c, d, ]
    // slice pattern: [first, second], [first, _, third], [first, .., nth], []
    // struct pattern: Color(r, g, b), Point {x, y}, Cart {suit: Clubs, rank: n}
    // reference: &value, &(k, v) -> matches only references
    // multiple patterns: 'a' | 'A'
    // Guard expressins: x if x * x <= r2

    // apart from enums other types can be matched too,
    let value = 1;
    match value {
        // the match value is moved or copied into the variable name
        0 => {}
        1 => println!("A Value is One"),
        n => println!("a Value is {n}"),
    }

    // this works because i32 implements Copy trait so it is copied instead
    println!("After Match Expression: {value}");

    // when matching a large struct and you only care for a small subset of the fields use .. to show you want to ignore the rest
    #[derive(Debug, Default)]
    #[allow(dead_code)]
    struct Account {
        id: String,
        email: String,
        fullname: String,
        date_created: String,
        date_updated: String,
        emergency_contact: String,
    }

    let possible_account = Some(Account {
        id: "001".to_string(),
        ..Default::default()
    });
    match possible_account {
        Some(Account { id, .. }) => println!("Found Account with id: {id}"),
        None => println!("Account Not Found"),
    }

    // slice are similar to array pattern only that slices are variable lengths

    println!("{}", "-".repeat(100));

    #[allow(clippy::useless_conversion)]
    let next_char = "Brian".to_string().chars().into_iter().next();
    match next_char.unwrap() {
        '0'..='9' => println!("Found a Number"),
        'a'..'z' => println!("Found a Small Letter"),
        'A'..='Z' => println!("Found a Capital Letter"),
        _ => println!("Found Something Else"),
    }

    // for @ matches a pattern and then binds that pattern to a variable
    match next_char.unwrap() {
        ch @ '0'..='9' => println!("Found a Number: [{ch}]"),
        ch @ 'a'..'z' => println!("Found a Small Letter: [{ch}]"),
        ch @ 'A'..='Z' => println!("Found a Capital Letter: [{ch}]"),
        _ => println!("Found Something Else"),
    }

    // patterns are also allowed in place of an identifier
    // such as variable declaration
    // function argument
    // iteration over keys and values
    //
    // these patterns are alwaus guaranteed to match and they are called irrefutable patterns
    // other patterns with no guarantees are not allowed in these contexts
    let account = Account {
        id: "002".to_string(),
        ..Default::default()
    };

    let Account { id, email, .. } = account;
    println!("ID: {id}");
    println!("Email: {email}");

    let another_account = Account {
        id: "003".to_string(),
        ..Default::default()
    };
    println!("Another Account: {another_account:?}");
    println!(
        "Another Account fields: id: {}, fullname: {}",
        another_account.id, another_account.fullname
    );

    #[allow(unused_assignments)]
    let ref value @ Account {
        ref id,
        ref fullname,
        ..
    } = another_account;

    // refutable patterns are allowed in match expressions, if let and while let
    println!("Value = {:?}", value);

    impl<T: PartialOrd> BinaryTree<T> {
        fn new() -> Self {
            Self::Empty
        }

        fn add(&mut self, value: T) {
            match *self {
                BinaryTree::Empty => {
                    *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                        element: value,
                        left: BinaryTree::Empty,
                        right: BinaryTree::Empty,
                    }))
                }
                BinaryTree::NonEmpty(ref mut tree_node) => {
                    if value <= tree_node.element {
                        tree_node.left.add(value);
                    } else {
                        tree_node.right.add(value);
                    }
                }
            }
        }
    }

    let mut root = BinaryTree::new();
    root.add(32.12);

    Ok(())
}
