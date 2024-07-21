#![allow(dead_code)]

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
    let mut fib: Fibonnaci = Fibonnaci { 
        curr: 1, 
        next: 1,
    };
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
    let mut persons_iter  = persons.iter();

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

    let numbers : Vec<i8> = (1..20).filter(|x| x % 2 == 0).collect();
    println!("Even Numbers = {:?}", numbers);
    
    let steps = vec![1, 2, 3, 4, 5];
    let pairs: Vec<_> = steps.iter().enumerate().collect();
    println!("Pairs = {:?}", pairs);

    // in addition to the iterators adapters the Iterator traits provide methods to
    // directly apply actions to the item in the data source sequence

    // for_each
    steps.iter().for_each(|x: &i32|  {
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
            Book { title: String::from("How to make money"), author: String::from("Ian Rives"), year: 2000 },
            Book { title: String::from("How to make a name"), author: String::from("Ian Rives"), year: 1920 },
            Book { title: String::from("How to make it"), author: String::from("Ian Rives"), year: 2012 },
            Book { title: String::from("How to make Family"), author: String::from("Ian Rives"), year: 1980 },
            Book { title: String::from("How to make Good Name"), author: String::from("Ian Rives"), year: 1999 },
            Book { title: String::from("How to make Love"), author: String::from("Ian Rives"), year: 2023 },
            Book { title: String::from("How to make Joy"), author: String::from("Ian Rives"), year: 2020 },
            Book { title: String::from("How to make Cars"), author: String::from("Ian Rives"), year: 2015 },
        ]
    }

    let books = collection();

    let max_years = books.iter().map(|book: &Book| {
        println!("Book year = {}", book.year);
        book.year
    }).max();

    println!("Years = {:?}", max_years);

    // it should be noted that the default behavior of the for in loop syntax
    // is to convert collection to an iterator using the into_iter method, which
    // consumes the item in the collection so after the iteration, the collection is no longer
    // available

    let some_collection = vec![1, 2, 3,  4, 5];
    let into_iter = some_collection.into_iter();
    
    for item in into_iter {
        println!("Item = {item}");
    }
    
    let some_collection = vec![1, 2, 3,  4, 5];
    let iter = some_collection.iter();

    for item in iter {
        println!("Item = {item}");
    }

    let mut some_collection = vec![1, 2, 3,  4, 5];
    let iter_mut = some_collection.iter_mut();

    for item in iter_mut {
        println!("Item = {item}");
    }


} 