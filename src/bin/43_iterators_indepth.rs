fn main() -> std::io::Result<()> {
    fn _triangle(n: i32) -> i32 {
        let mut sum = 0;
        // the expression 0..=n is a RangeInclusive<i32> and this is an iterator that produces integers in the range 0-> n including n
        for i in 0..=n {
            sum += i;
        }
        sum
    }

    println!("Triange @ 5 = {}", _triangle(5));

    // iterators can be used in a for loop as shown above, but they also have methods like fold
    fn _triangle_v2(n: i32) -> i32 {
        #[allow(clippy::unnecessary_fold)]
        (1..=n).fold(0, |sum, item| sum + item)
    }

    println!("Triange V2 @ 5 = {}", _triangle_v2(5));

    // an iterator is any value that implements the Iterator trait
    //
    /*

    trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    trait IntoIterator {
        type Item;
        item IntoIter: Iterator<Item=Self::Item>;

        fn into_iter(self) -> Self::IntoIter;
    }

    */

    // we call any type that implements the IntoIterator an iterable
    // an iterator is any type that implement the Iterator trait
    // An interator produces values
    // the code that receives the item produced by an iterator is called a consumer, the for loop is usually the consumer

    // all iterators automatically implement IntoIterator that return themselves
    #[allow(unused_mut)]
    let mut cities = vec!["monaco", "venice", "uyo", "abak", "calabar"];
    for city in IntoIterator::into_iter(&cities) {
        println!("City: {city}");
    }

    // println!("Cities: {cities:?}");
    // most collection has an iter() and iter_mut() method to get a natural iterator from them
    // the iter() method returns a iterator over shared references to the items in the collection
    // iter_mut() return mutable reference over the items in the collection
    //
    let v = vec![12, 23, 45, 56, 67, 78, 89, 90, 100];
    let mut iter = v.iter();

    assert_eq!(iter.next(), Some(&12));
    assert_eq!(iter.next(), Some(&23));
    assert_eq!(iter.next(), Some(&45));
    assert_eq!(iter.next(), Some(&56));
    assert_eq!(iter.next(), Some(&67));
    assert_eq!(iter.next(), Some(&78));

    // for some types, with more than one way to iterate through them, they can provide different methods to
    // handle the different ways of iterating through them
    // an example being the &str, you can iterate over it characters or over it bytes
    // &str.bytes(), &str.chars();
    //
    let name = "Brian David Obot";
    for char in name.chars() {
        println!("Char: {char}");
    }

    for byte in name.bytes() {
        println!("Byte: {byte}");
    }

    use rand::random;
    use std::iter::from_fn;

    let lengths = from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
        .take(1000)
        .collect::<Vec<f64>>();

    println!("Length: {lengths:?}");

    fn fibonacci() -> impl Iterator<Item = usize> {
        let mut state = (0, 1);
        std::iter::from_fn(move || {
            state = (state.1, state.0 + state.1);
            Some(state.0)
        })
    }

    let result = fibonacci().take(8).collect::<Vec<_>>();
    println!("Result: {result:?}");

    // the drain methods: this takes ownership of the elements in the collection and leave the collection empty at the end of the drain
    // Read up more about the drain method here
    //
    // std::ops::Range Iterator -> 1..10
    // std::ops::RangeFrom -> 1..
    // std::ops::RangeInclusive -> 1..=10
    // Option<T>: Can be iterated over and behaves like a vector whose length is either 0 (None) or 1 (Some(T))
    let some_value = Some(10);
    #[allow(for_loops_over_fallibles)]
    for value in some_value {
        println!("✅ {value:?}");
    }

    // Result<T, E>: Similar to the Option
    // Vec<T>, &[T]
    let values = (0..=100).collect::<Vec<_>>();
    println!("Values: {values:?}");

    for (index, window) in values.windows(10).enumerate() {
        println!("Window {index}: {window:?}");
    }

    for (index, chunk) in values.chunks(10).enumerate() {
        println!("Chunk {index}: {chunk:?}");
    }

    for item in values.split(|byte| byte & 1 == 0) {
        println!("Item = {item:?}");
    }

    // String, &str
    let sentence =
        String::from("Brian David Obot Is the Maintainer for the Rust Learning Repository");

    for byte in sentence.bytes() {
        println!("Bytes: {byte:?}");
    }

    for char in sentence.chars() {
        println!("Char: {char}");
    }

    for word in sentence.split_whitespace() {
        println!("Words: {word}");
    }

    for line in sentence.lines() {
        println!("Line: {line}");
    }

    for section in sentence.split("Is") {
        println!("Section: {section}");
    }

    // std::collections::HashMap
    let hobbies = std::collections::HashMap::from([
        ("Brian", vec!["Coding", "Chess", "Video Games", "Eating"]),
        ("James", vec!["Eating"]),
        ("Tony", vec!["Coding", "Eating"]),
        ("Id", vec!["Coding"]),
    ]);

    for name in hobbies.keys() {
        println!("Name: {name}")
    }

    for hobby_list in hobbies.values() {
        println!("Hobbies: {hobby_list:?}");
    }

    #[allow(unused_imports)]
    use std::io::Read;

    // std::io::Read
    let _file = std::fs::File::open("sample_file.txt").unwrap();

    // for byte in file.bytes() {
    //     println!("Byte: {byte:?}");
    // }

    // Adapters are methods that consume an iterator and build one with useful methods
    // map: Transform an iterator by applying a closure to it items

    let double_numbers = (0..100).map(|item| item * 2).collect::<Vec<_>>();
    println!("Numbers: {double_numbers:?}");

    // filter applies a closure to the iterator and filter out elements that do not return true on the closure
    let even_numbers = (0..100).filter(|item| item & 1 == 0).collect::<Vec<_>>();
    println!("Even Numbers: {even_numbers:?}");

    // when adapters are use used, they are not called until the iterator they produce is actually used (next is called on it)
    #[allow(unused)]
    ["earth", "water", "air", "fire"]
        .iter()
        .map(|elt| println!("{elt}"));

    // other adapters include
    // filter_map and flat_map
    let suspects = (0..10)
        .filter_map(|item| {
            if (item & 1) == 0 {
                Some(item * 2)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    println!("SUspects: {suspects:?}");

    // take and take_while lets you end an iteration after a certain number of items
    // take while applies a predicate and only ends when the predicate return false
    let first_ten = (0..100).take(10).collect::<Vec<_>>();
    println!("First Ten: {first_ten:?}");

    let under_50: Vec<i32> = (0..100).take_while(|item| item / 50 < 1).collect();
    println!("Under 50: {under_50:?}");

    // skip and skip_while
    // they drop a certain number of elements at the start of an iteration
    let above_50: Vec<_> = (0..100).skip(50).collect();
    println!("Above 50: {above_50:?}");

    // peekable; returns an iterator with the ability to peek at the next item in the iteration
    // fuse: takes an iterator and return one that would alwaus return None when it has reached the end and it\s next method is called again

    // rev: some iterators are able to draw item from the end of the iterator
    // Only iterators that implement DoubleEndedIterator have access to the rev method
    let reverse: Vec<i32> = (0..10).rev().collect();
    println!("Reverse: {reverse:?}");

    // inspect: use to debug an iterator pipeline
    // chain: appends one iterator to another, but both iterators have to have the same Associated Item type
    // a chain iterator is reversible is both iterators are reversible too

    Ok(())
}
