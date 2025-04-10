#[allow(unused)]

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 5];
    let threshold = 3;
    let filtered_numbers = numbers.into_iter().filter(|x| x >= &threshold).collect::<Vec<_>>();
}