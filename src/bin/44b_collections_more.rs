use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let nums = [9, 3, 7, 1, 8, 2, 6, 4, 5];
    let k = 4;

    // TODO: build a min-heap with Reverse and pop the k smallest in order
    let mut heap = BinaryHeap::from_iter(nums.into_iter().map(Reverse));

    let mut smallest: Vec<i32> = Vec::new();
    for _ in 0..k {
        let Reverse(n) = heap.pop().unwrap();
        smallest.push(n);
    }
    let s: Vec<String> = smallest.iter().map(|v| v.to_string()).collect();
    println!("{}", s.join(" "));
}
