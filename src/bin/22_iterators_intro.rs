#![allow(dead_code)]

// iterators is something you can call the next method on repeatedly and it gives us a sequence of things
// technically this means iterators implement the Iterator trait which in turns implement the
// next method

// a range like 0..10 is mathematically represented as "[0, 10)",
// inclusive on the left and exclusive on the right

fn main() {
    let vec_1 = vec![1, 2, 3, 4];
    let vec_iter = vec_1.iter();

    for value in vec_iter {
        println!("Got {}", value);
    }

    struct Fibonnaci {
        curr: i32,
        next: i32,
    }

    impl Iterator for Fibonnaci {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            let new_next = self.curr + self.next;

            self.curr = self.next;
            self.next = new_next;

            Some(self.next)
        }
    }

    // notice how i made the reference to fib a mutable reference
    // this is so because in the trait implmentation next, the struct is being mutated
    let mut fib: Fibonnaci = Fibonnaci { curr: 1, next: 1 };
    println!("I = {:?}", &fib.curr);
    for _ in 0..10 {
        println!("I = {:?}", &fib.next());
    }

    // once an iterator is created, it is lazy, it has no effect until it is used
    let v2: Vec<i32> = (1..100).collect();

    let _v2_iter = v2.iter();

    // println!("V2 = {:?}", v2);
    // println!("V2 iter = {:?}", v2_iter);

    // implicitly when we loop over an array in a for loop
    // an iterator is created from the array and iterated over

    for i in 1..10 {
        println!("Got {i}");
    }
    // loops takes ownership of iterators and make them mutable behind the scene

    // more on the Iterator trait and the next method
    // each call to next consumes an item from the iterator

    let persons = vec![1, 2, 3, 4, 5];

    // it is important to always declare iterators as mutable since a consuming
    // changes their internal states, this is abstracted away in for loops
    let mut persons_iter = persons.iter();

    // after this line the first item in the iterator is consumed, move into the first_person variable
    let first_person = persons_iter.next();

    println!("First person = {:?}", first_person);
    println!("Persons iter = {:?}", persons_iter);

    for person in persons_iter {
        println!("Person = {:?}", person);
    }

    // note the values we get from calling next are immutable references to
    // the value in the sequence the iterators were generated from or for

    // if you want to get a mutable reference to the item in the sequence in the iterators
    // use into_mut instead of iter method, and if you want to get own values in the iterator
    // use into_iter instead of iter method

    // sum method in the Iterator trait implementation
    // methods that call next are called consuming adaptors, because the use up the iterator
    let sum: i32 = persons.iter().sum();
    let min: i32 = persons.iter().min().unwrap_or(&0).to_owned();
    let max: i32 = persons.iter().max().unwrap_or(&0).to_owned();

    println!("Sum = {sum}");
    println!("Min = {min}");
    println!("Max = {max}");

    // iterator adapters are methods defined in the iterator trait that do not consume
    // the iterator but produce other iterators by changing some aspect of the original iterator
    // an example if the map method

    let ages = vec![1, 2, 3, 4, 5, 6];
    // iterators adapters are lazy and must always be consumed,
    // i consume mine with the collect method
    let double_ages: Vec<i8> = ages.iter().map(|age| age * 2).collect();

    println!("Ages = {:?}", ages);
    println!("Double ages = {:?}", double_ages);

    // filter method takes a closure that evaluate to a boolean
    // and the value passed to the closure, is included in the new iterator if
    // the boolean is truthy

    let numbers: Vec<i8> = (1..20).filter(|x| x % 2 == 0).collect();

    let first_five = (0..100).take(5).collect::<Vec<_>>();
    println!("First Five = {:?}", first_five);

    // the collect method converts any iterator into a collection and typically
    // need to specify the type of the collection we want to generate with the collect
    // the collect method can be used directly to convert one collection to another type

    println!("Even Numbers = {:?}", numbers);

    let steps = vec![1, 2, 3, 4, 5];
    let pairs: Vec<_> = steps.iter().enumerate().collect();
    println!("Pairs = {:?}", pairs);

    // the collect consumer can receive the type to collect to using the turbo fish syntax
    let sum = (0..10).collect::<Vec<i32>>();
    println!("SUm = {sum:?}");

    // another consumer is the find consumer, which takes in a closure and return the first element that
    // mtaches the closure logic

    let mut first_ten_numbers = 0..11;

    let first_odd_num = first_ten_numbers.find(|x| x % 2 != 0).unwrap();
    let first_even_num = first_ten_numbers.find(|x| x % 2 == 0).unwrap();
    println!("First ODD Number = {first_odd_num}");
    println!("First Even Number = {first_even_num}");

    // the fold is an interesting consumer and it can be a little confusing but here goes nothing
    // the structure of the fold call is as follows
    // .fold(base, |accumulator, x| <logic to accumulate accumulator>)

    let sum = (0..10).fold(0, |sum, x| sum + x);
    // there is beauty in high and low places - Brian

    let multiple = (1..11).fold(1, |mul, x| mul * x);

    println!("Fold in Sum Call = {sum}");
    println!("Fold in Mul Call = {multiple}");

    // in addition to the iterators adapters the Iterator traits provide methods to
    // directly apply actions to the item in the data source sequence

    // for_each
    steps.iter().for_each(|x: &i32| {
        println!("Result = {x}");
    });

    // count
    let count = steps.iter().count();
    println!("Steps Count = {count}");

    // len
    let len = steps.iter().len();
    println!("Len of Steps = {len}");

    struct Book {
        title: String,
        author: String,
        year: i32,
    }

    fn collection() -> Vec<Book> {
        vec![
            Book {
                title: String::from("How to make money"),
                author: String::from("Ian Rives"),
                year: 2000,
            },
            Book {
                title: String::from("How to make a name"),
                author: String::from("Ian Rives"),
                year: 1920,
            },
            Book {
                title: String::from("How to make it"),
                author: String::from("Ian Rives"),
                year: 2012,
            },
            Book {
                title: String::from("How to make Family"),
                author: String::from("Ian Rives"),
                year: 1980,
            },
            Book {
                title: String::from("How to make Good Name"),
                author: String::from("Ian Rives"),
                year: 1999,
            },
            Book {
                title: String::from("How to make Love"),
                author: String::from("Ian Rives"),
                year: 2023,
            },
            Book {
                title: String::from("How to make Joy"),
                author: String::from("Ian Rives"),
                year: 2020,
            },
            Book {
                title: String::from("How to make Cars"),
                author: String::from("Ian Rives"),
                year: 2015,
            },
        ]
    }

    let books = collection();

    let max_years = books
        .iter()
        .map(|book: &Book| {
            println!("Book year = {}", book.year);
            book.year
        })
        .max();

    println!("Years = {:?}", max_years);

    // it should be noted that the default behavior of the for in loop syntax
    // is to convert collection to an iterator using the into_iter method, which
    // consumes the item in the collection so after the iteration, the collection is no longer
    // available

    let some_collection = vec![1, 2, 3, 4, 5];
    let into_iter = some_collection.into_iter();

    for item in into_iter {
        println!("Item = {item}");
    }

    let some_collection = vec![1, 2, 3, 4, 5];
    let iter = some_collection.iter();

    for item in iter {
        println!("Item = {item}");
    }

    let mut some_collection = vec![1, 2, 3, 4, 5];
    let iter_mut = some_collection.iter_mut();

    for item in iter_mut {
        println!("Item = {item}");
    }

    // iterators have an nth method which returns the nth element in the sequence of the iterator
    let numbers = [1, 2, 3, 4, 5];
    let numbers_iter = numbers.iter();

    for num in numbers_iter {
        println!("NUM: {}", num);
    }

    let zeroth_element = numbers.iter().nth(0);
    let oneth_element = numbers.iter().nth(1);
    println!("Zeroth Element: {}", *zeroth_element.unwrap_or(&0));
    println!("Oneth  Element: {}", *oneth_element.unwrap_or(&0));

    // when given two iterators we can chain them together to form a new longer iterator
    // this chain iterator might not always be collectable into a single collection type, since
    // the items in each iterator might be of different types, so the chain works best when you want to maybe iterate
    // of the longer list
    let first_iter = 0..=100;
    let second_iter = 101..=200;

    let chained = first_iter.chain(second_iter);
    // the for each method is similar to the map method on the iterator but it does not change each item in the
    // iterator but simply runs an action for each item as the name suggests
    // for each is equivalent to the for loop, and might be faster in some cases
    // NOTE: we can not use break and continue in closure as we can use in the normal for loop syntax
    chained.for_each(|x| println!("Item from chain = {x}"));

    // we can also zip pairs from two iterators into a tuples
    let first_iter = 0..=10;
    let second_iter = 11..=20;

    let pairs = first_iter.zip(second_iter).collect::<Vec<_>>();
    println!("Pairs: {:#?}", pairs);

    // we can use the skip method to skip certain items in an iterator too
    // the skip methods return an iterator which skips the number of items at the start of the
    // base iterator it was applied on
    let some_iter = 0..5;
    println!(
        "Did not skip this items: {:?}",
        some_iter.skip(1).collect::<Vec<_>>()
    );

    // we can also use the take method on iterator, it is similar to skip
    // it instead of discarding the number it items from the start, it only uses those item
    // just like skip, this also returns an iterator, it is a adapter  method,
    let first_three_items = (0..1000).take(3);
    println!("First Three Items = {:?}", first_three_items);

    // peek into the next item without advancing the iterator state
    let numbers = 0..=10;
    let mut peekable = numbers.peekable();
    let first_number = peekable.next();
    let peek_second_number = peekable.peek();
    println!("First Number = {:?}", first_number.unwrap());
    println!("Peek Second Number = {:?}", peek_second_number.unwrap());
}
