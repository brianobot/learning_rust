// Brian's Test
use std::time::Instant;

fn main() {
    let start = Instant::now();
    run();
    println!("Duration: {:?}", Instant::now() - start);
}


fn run() {
    for i in 0..1_000_000_000i128 {
        let _a = (i * 1_000_000) as f64 / 1.3243;
    }
}

// cargo run --package guessing_game --bin a_biilion_iterations --release