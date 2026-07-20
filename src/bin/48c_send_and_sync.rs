use std::{sync::Arc, thread};

fn main() {
    // Send data types can be sent across thread
    let string = String::from("Data");
    let handle = thread::spawn(move || {
        println!("{string}");
    });

    handle.join().unwrap();

    // Sync data types can be referenced from multiple threads
    // the Arc is placed on the heap and a reference count it maintained
    let data = Arc::new(5);

    let clone1 = Arc::clone(&data);
    let clone2 = Arc::clone(&data);

    let handle_2 = thread::spawn(move || {
        println!("{}", clone1);
    });

    let handle_3 = thread::spawn(move || {
        println!("{}", clone2);
    });

    handle_2.join().unwrap();
    handle_3.join().unwrap();

    // T is Sync if &T is Send
}
