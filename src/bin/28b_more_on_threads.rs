use std::time::Duration;
use std::thread;

fn main() {
    let process = || {
        for i in 0..=10 {
            println!("Thread processs @ {i}");
            thread::sleep(Duration::from_millis(10))
        }
    };

    let handle = thread::spawn(process);

    for i in 0..=10 {
        println!("Main Thread @ {i}");
        thread::sleep(Duration::from_millis(10))
    }

    handle.join().unwrap();
}