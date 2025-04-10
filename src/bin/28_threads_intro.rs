use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    // a thread is a way to manage an independent flow or execution of part of a program
    // in rust, there is a 1:1 relationship between the thread created in rust and the thread
    // create in the underlying operating system, to create a thread, we use the spawn method and
    // pass it a closure which would be executed in the thread

    // a spawned thread without a captured handle is a detached thread, since there is no way to know
    // when it has completed or failed
    thread::spawn(|| {
        for i in 0..100 {
            println!("Hi number {i} from spawned thread 1");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // there is no parent error propagation between spawned thread and the spawining thread
    // the spawned may outlive the spawning thread unless the spawning thread is the main thread

    // a new thread can be configured before it is spawned, it name and stack size can be set
    let another_handle = thread::Builder::new()
        .name("thread1".to_string())
        .spawn(move || {
            println!("Hello, world!");
        });

    let _res = another_handle.unwrap();
    println!("Response from Built Thread: {:?}", _res);

    thread::spawn(|| {
        for i in 0..100 {
            println!("Hello Number {i} from spawned thread 2");
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..=5 {
        println!("Hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(10));
    }

    // when the main thread of a rust program completes all spawned threads are shut down immediately
    // whether or not they have finished running, there is also no guarantee in the order of the thread execution
    // and no guarantee that the threaded part would even get the chance to run at all, to ensure this we would employ
    // the join method

    let handle = thread::spawn(|| {
        for i in 0..=200 {
            println!("Hello There number {i} from thread 3");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
    // this call here blocks the thread currently running until the thread represented by the handle finishes
    // blocking means it is prevented from doing work or from terminating

    // when using varaibles from the environment, the thread must move the variable since a borrow would be problematic
    // since the thread might outlive the environment from whcih the varaible was borrowed from at which point the
    // variable might be invalid

    let data = vec![1, 2, 3, 45];

    let handle_2 = thread::spawn(move || {
        println!("Data moved from the main thread = {:?}", data);
    }); // at this point the data variable is no longer available in the main thread, since it has been moved
        // into the handle_2 thread

    handle_2.join().unwrap();

    let (tx, rx) = mpsc::channel();
    // each channel sends a partcular data type alone

    thread::spawn(move || {
        let val = String::from("Brian David Obot");
        let result = tx.send(val).unwrap(); // the send method would fail is the receiver has dropped
                                            // for this reason the send method returns a Result<T, E>
                                            // at this point the val variable has been moved out of this scope and would be available
                                            // at whereever is the receiving end of the channel
        println!("Result of Transmission: {:?}", result);
    }); // at this point, the tx variable as been moved into the thread

    let received = rx.recv().unwrap(); // the recv method would block the thread until a value is received
                                       // once the transmitter closes the recv would return an error since no more value can be received
    println!("Received = {:?}", received);

    // if we do no want to block the thread while checking for messages on the channel
    // we can use the try_recv, this would return a value immediately and an error is no value is available

    // an example to show snending multiple messages and watching the receiver waiting for each one
    let (tz, rz) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hello there"),
            String::from("My name is Brian"),
            String::from("I am a budding rust developer"),
            String::from("I look forward to hearing about you"),
        ];

        for msg in vals {
            tz.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rz {
        println!("GOT: {received}");
    }

    // we can create multiple producers by cloning the transmitter
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let val = String::from("message from thread 1");
        tx1.send(val).unwrap();
    });

    thread::spawn(move || {
        let val = String::from("Messagre from thread 2");
        tx.send(val).unwrap();
    });

    for received in rx {
        println!("Received: {received}");
    }

    // passing message is not the only way to handle concurrency in rust
    // we can communicate by sharing memory between threads,
    // channels is a single ownership model, where each thread explicitly owns or gives away
    // a piece of data when transmitting it, in the shared memory approach,

    // the can be achieved with mutex (Mutual Exclusion), where only one thread is allowed to access some data at aany given time
    // to access the data in a mutex,the thread must first signal that it wants access by asking to acquire the mutex' lock
    // the lock is a data structure that is part of the mutex that keeps tracj of who currentlys has exclusive acces to the data.

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        // calling lock on the mutex would block the thread until it is the thread's turn to have the lock
        // so lock would essentially wait until the lock available for use, lock methods returns a result
        // which can contain an error in the case that the previous holder panicked, after the lock is acquired,
        // the returned wrapped value can be treated as a reference to the value contained in the mutex

        // more technically, the call to lock returns a MutexGuard smart pointer that implements a Deref to the value wrapped as
        // the inner data, the MutexGuard implements a drop implementation that releases the lock automatically when the Mutexguard
        // goes out of scope as in our case when the scope is ended
        *num = 6;
    }

    println!("m = {m:?}");

    // we can get the thread type through two ways
    let handle = thread::spawn(|| {});
    println!("A Thread = {:?}", handle.thread());

    let current_thread = thread::current();
    println!("Current thread = {:?}", current_thread);
}
