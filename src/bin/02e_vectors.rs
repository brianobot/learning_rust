fn main() {
    let mut data = vec![0; 5]; // Create a vector of 5 elements, all initialized to 0
    println!("{:?}", data);

    // you can push elements to the vector
    data.push(1);
    println!("{:?}", data);

    // you can pop elements from the vector
    let last_value = data.pop();
    if last_value.is_some() {
        println!("Popped value: {}", last_value.unwrap());
    }

    // you can access elements by index
    let first_value = data[0];
    println!("first value = {}", first_value);

    // indexing to an invalid address would cause the program to panic
    // let some_valid_point = data[100];

    // slices a read-only
    let some_slice = &data[1..data.len()];
    println!("Some slice = {:?}", some_slice);

    // vectors length are different from their capacity
    // when the length exceeds the capacity, the vector is reallocated
    // to prevent reallocation, you can use the `with_capacity` method
    // an empty vector has a capacity of 0 and no heal allocation
    println!("Length = {}, Capacity = {}", data.len(), data.capacity());

    let empty_vector: Vec<i32> = vec![];
    let another_empty_vector = Vec::<i32>::new();

    println!("Empty vector capacity = {:?}", empty_vector.capacity());
    println!("Another empty vector capacity = {:?}", another_empty_vector.capacity());

    // creating a vector with a certain capacity
    let mut some_names = Vec::<String>::with_capacity(10);
    some_names.push("John".to_string());
    some_names.push("Doe".to_string());
    println!("Some names = {:?}", some_names);

    let element_vector = vec![1, 2];
    println!("element vector capacity: {}", element_vector.capacity());


}
