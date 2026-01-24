use std::fmt::Debug;

fn main() {
    // Rust is an expression language
    let allowed = false;
    let name = if allowed {
        "Brian".to_string()
    } else {
        "Unknown".to_string()
    };

    println!("Name: {name}");
    // if expressions can be used like another other expression
    // match expressions can also be used as such
    // they can be passed as argument to macros and functions
    // control flow in rusts are expressions
    //
    // a block produces a value can be used anywhere a value is required
    #[derive(Debug)]
    struct Post {
        #[allow(dead_code)]
        title: Option<String>,
        author: Option<String>,
    }

    impl Post {
        fn new(title: Option<String>, author: Option<String>) -> Self {
            Self { title, author }
        }

        fn author(&self) -> Option<String> {
            self.author.clone()
        }
    }

    let post = Post::new(Some("New Post".to_string()), Some("Brian".to_string()));
    let display_name = match post.author() {
        Some(author) => author, // after => is a simple expression author
        None => {
            // after => is a block expression surrounded with a curly braces
            String::from("Unknown")
        }
    };

    println!("Display Name: {}", display_name);
    dbg!(post);

    // a block can contain item declaration such as Struct, functions or use
    // any block may contain a function
    #[allow(dead_code)]
    fn show_files() -> std::io::Result<()> {
        #[allow(dead_code)]
        let mut _v = vec![1, 2, 3];

        // perfectly valid rust code
        // fn cmp_by_timestamp_then_name(a: &FileInfo, b: &FileInfo) -> Ordering {
        //
        // }
        //
        Ok(())
    }

    // all blocks of an if expression must produce values of the same type
    // similarly all arms of a match expression must have the same type
    //
    // if let pattern = expr {
    // } else {
    //
    // in the if let block, if the expr matches the pattern, the if block is ran else the optional else block is ran
    // sometimes this is a nice way to get values out of an Option type
    let result: Option<u8> = Some(10u8);
    if let Some(value) = result {
        println!("Value is {value}");
    } else {
        println!("Value is Not Available");
    }

    // if let is a syntactic sugar for match, match expression can do everything if let can do

    // loops: they are 4 types of loops in rust
    // 1: while condition { ... }
    // 2: while let pattern { ... }
    // 3: loop { ... }
    // 4: for pattern in iterable { ... }
    //
    // loops are expressions in rust, but the value of a while loop or a for loop is always ()
    let mut start = 10u8;
    let value = loop {
        println!("Current Value = {start}");
        start += 1;

        if start % 24 == 0 {
            break start; // you can use break to return values from a loop in rust
        }
    };

    println!("Loop Produced the value {value}");
    // for loops consume values that they iterate over by default
    //
    let strings = vec!["boy", "girl", "child", "toy"];
    for s in strings {
        println!("s = {s}");
    } // the

    // println!("Strings: {strings:?}");// trying to access the vector here is an error

    // break can be used to break a loop
    // continue is used to jump to the next iteration of a loop
    //
    // a loop can be labelled with a lifetime
    let mut temp = 10;
    'search: loop {
        println!("Temp: {temp}");
        for i in 0..temp {
            print!("{i}");
            if i == 35 {
                break 'search;
            }
        }
        println!("");
        temp += 2;
    }
    println!("");

    // a break can have a label and a value but both are optional
    // break - empty break
    // break 'label
    // break 'label value
    // break value
    //

    // labels can be used with continue
    // returning without a value is shorthand for return ();
    //
    let value = 10i32;
    let r = &value;

    println!("R: {}", r.abs());

    // the .. operator produces 4 different types of range types based on it use
    // .. RangeFull
    // a.. RangeFrom
    // ..b RangeTo
    // a..b Range
    //
    // there is 2 more
    // ..=b RangeToInclusive
    // a..=b RangeInclusive
    //
    // only ranges that include a start value are iterable, since the iteration must start from somewhere
    let mut chaos = vec![2, 4, 1, 3, 5, 6];
    println!("Chaos = {chaos:?}");
    quicksort(&mut chaos);
    println!("Calm = {chaos:?}");

    // Integers divions round towards zero
    // there is no Unary operator +
    // % can be used on floating point numbers too
    let x = 1234.567 % 10.0;
    println!("X = {x}");

    // values that implement the Deref allows the usuage of the Pointer to be much like using the underlying value
    // usong Box<ChessBoard> is mostly just like using a plain ChessBoard
}

fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let slice_len = slice.len();
    let pivot_index = slice_len - 1;
    let mut i = -1isize;

    for j in 0..pivot_index {
        if slice[j] <= slice[pivot_index] {
            i = i + 1isize;
            slice.swap(i as usize, j);
        }
    }

    // this moves the pivot to the correct place
    i += 1;
    slice.swap(i as usize, pivot_index);
    i as usize
}

fn quicksort<T: Ord + Debug>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }

    let pivot_index = partition(slice);
    quicksort(&mut slice[..pivot_index]);
    quicksort(&mut slice[pivot_index..]);
}
