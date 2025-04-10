extern crate chrono; // extern is used to make rust aware of the dependency on an external crate
use std::time::{Duration, Instant};

fn main() {
    let dur1 = Duration::from_secs(15);
    println!("Dur1: {dur1:?}");

    let dur2 = Duration::from_millis(5500);
    println!("Dur2: {:?}", dur2.as_secs());

    let diff = dur1 - dur2;
    println!("Difference = {:?}", diff);

    // we can time the time elapsed by using the Instant struct
    let now = Instant::now();
    // some slow operation operation
    std::thread::sleep(Duration::from_secs(2));

    let elapsed_time = now.elapsed();
    println!("Elapsed Time = {:?}", elapsed_time);

    let now = chrono::Utc::now();
    println!("Now = {:?}", now);
}
