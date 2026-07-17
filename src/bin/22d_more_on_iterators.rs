fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];

    let v_iter = v.iter();

    for item in v_iter {
        println!("Item: {item}")
    }

    for item in v {
        println!("I: {item}");
    }

    // methods that call the next method in iterators are called consumers
    // methods that do not call next method are called adapters
    let sum_of_values = [1, 2, 3, 4, 5].iter().sum::<i32>();
    println!("Sum of values = {sum_of_values}");

    // after using a consumer, the iterator is well, CONSUMED
    // adapters are methods that produce other iterators
    // in this case, the map method is an adapter method
    let _v2_iter = [2, 3, 4, 5].iter().map(|x| x * x).collect::<Vec<_>>();

    // let sum_of_squares: i32 = v.iter()
    //     .filter(|&x| x % 2 == 0)
    //     .map(|x| x*x)
    //     .sum();
}
