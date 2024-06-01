use std::collections::HashMap;

fn _median(_list: &Vec<i32>) -> i32 {
    let len = _list.len();
    if len == 0 {
        return 0;
    }
    let divisble_by_2 = len % 2 == 0;
    match divisble_by_2 {
        true => _list[(((len / 2) + ((len/ 2) + 1))/2) - 1],
        false =>  _list[((len + 1) / 2) - 1]
    }
} 

fn _mode(list: &Vec<i32>) -> Option<i32> {
    
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for number in list {
        let count = counts.entry(*number).or_insert(0);
        *count += 1;
    }

    let mut most_count = 0;
    let mut number_with_most_count = None;

    for (number, count) in counts {
        if count > most_count {
            most_count = count;
            number_with_most_count = Some(number);
        }
    }
    
    number_with_most_count
}

fn main() {
    let mut data = vec![1, 2, 3, 4, 5, 4];

    // inplace sorting here
    data.sort();

    let median = _median(&data);

    let mode = _mode(&data);

    println!("Data = {:?}", data);
    println!("Median = {median}");
    println!("Mode = {}", mode.unwrap_or(-1));
}