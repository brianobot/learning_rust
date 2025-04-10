use std::sync::mpsc;
use std::{thread, time::Duration};

fn main() {
    let data = vec![1, 2, 3, 4, 5, 5];
    // we need scope threads when we want to ensure that a group of threads run until completion before containing the
    // main or parent thread execution, at the end of the scope thread, there is an automatic join at the end of the
    // scope of the scoped thread, because scope threads are ensured to pause execution of the parent thread
    // we can safely borrow values from the environment unlike in the normal thread which we must move the value
    thread::scope(|scope| {
        scope.spawn(|| {
            thread::sleep(Duration::from_millis(10));
            println!("Thread 1");
        });

        scope.spawn(|| {
            thread::sleep(Duration::from_millis(10));
            println!("Thread 2");
        });

        scope.spawn(|| {
            thread::sleep(Duration::from_millis(10));
            println!("Thread 3");
        });

        scope.spawn(|| {
            thread::sleep(Duration::from_millis(20));
            println!("Data = {:?}", data);
        });
    });

    println!("This is from the Main thread!");
    println!("The Data Variable is still active here: {:?}", data);

    let (tx, rx) = mpsc::channel();

    let mut handles = vec![];
    for _i in 0..10 {
        let tx1 = tx.clone();
        let handle = thread::spawn(move || {
            tx1.send(_i).unwrap();
            _i
        });
        handles.push(handle);
    }

    for _i in 0..30 {
        thread::sleep(Duration::from_millis(10));
        match rx.try_recv() {
            Ok(value) => println!("Value: {:?}", value),
            Err(msg) => println!("Found Error: {:?}", msg),
        }
    }

    for handle in handles {
        let h1 = handle.join().unwrap();
        println!("Thread Return value: {:?}", h1);
    }
}
