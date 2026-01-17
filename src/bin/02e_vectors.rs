use std::vec;

fn main() {
    let mut data = vec![0; 5]; // Create a vector of 5 elements, all initialized to 0
    println!("{:?}", data);
    
    // vectors can also be created by repeating a certain value
    let another_vector = vec![0; 10 * 10];
    println!("✅ Another Vector: {another_vector:?}");

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
    println!(
        "Another empty vector capacity = {:?}",
        another_empty_vector.capacity()
    );

    // creating a vector with a certain capacity
    let mut some_names = Vec::<String>::with_capacity(10);
    some_names.push("John".to_string());
    some_names.push("Doe".to_string());
    println!("Some names = {:?}", some_names);

    let element_vector = vec![1, 2];
    println!("element vector capacity: {}", element_vector.capacity());

    // vector associated functins and methods
    // new: creates a new vector

    let _new_vector = Vec::<i32>::new();

    // we can create higher dimensional vectors
    let mut square_board: Vec<Vec<i32>> = Vec::new();

    for i in 1..=3 {
        let row = vec![i; 3];
        square_board.push(row);
    }

    println!("Square board = {:?}", square_board);

    // can initialize a vector from an array
    let mut array = [1, 2, 3, 4, 5].to_vec();
    println!("Array to vector = {:?}", array);

    // the default behaviour when iterating over a vector is to consume the elements
    // for item in array {
    //     println!("Item = {}", item);
    // }

    // into_iter() consumes a vector and returns an iterator
    // after whihc the initial vector is no longer usable
    let array_vector_iter = array.iter();
    for item in array_vector_iter {
        println!("Item as reference = {}", item);
    }

    let array_vector_iter_mut = array.iter_mut();
    for item in array_vector_iter_mut {
        *item += 1;
        println!("Item as mutable reference = {}", item);
    }

    let array_vector_into_iter = array.into_iter();
    for item in array_vector_into_iter {
        println!("Item consumed = {}", item);
    }

    // we can emulate the behaviour of storing different types in a vector
    // by storying an enum that can hold different types

    #[derive(Debug)]
    enum FieldType {
        SmallIntegerField(i8),
        IntergerField(i32),
        BigIntergerField(i128),
        StringField(String),
        FloatField(f32),
    }

    impl FieldType {
        fn get_value(&self) -> Option<String> {
            match self {
                FieldType::SmallIntegerField(value) => Some(value.to_string()),
                FieldType::IntergerField(value) => Some(value.to_string()),
                FieldType::BigIntergerField(value) => Some(value.to_string()),
                FieldType::StringField(value) => Some(value.to_string()),
                FieldType::FloatField(value) => Some(value.to_string()),
            }
        }
    }

    let mut columns: Vec<FieldType> = Vec::new();

    columns.push(FieldType::IntergerField(10));
    columns.push(FieldType::SmallIntegerField(10));
    columns.push(FieldType::BigIntergerField(10000000));
    columns.push(FieldType::StringField("Hello".to_string()));
    columns.push(FieldType::FloatField(3.14));

    println!("Columns = {:?}", columns);

    for column in columns {
        let value = column.get_value();
        println!("Value = {:?}", value.unwrap());
    }

    // sorting an array can be done with .sort, .reverse and the sort_by methods
    let mut data = vec![
        45, 23, 4, 5, 23, 54, 23, 5, 2, 42, 31, 4, 3, 43, 5, 3, 3, 4, 3, 4, 35,
    ];
    data.sort();
    println!("Sorted data = {:?}", data);

    data.reverse();
    println!("Reversed data = {:?}", data);

    let mut names = vec!["John", "Doe", "Jane", "Doe", "Alice", "Bob"];
    names.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("Sorted names = {:?}", names);

    // we can get a slice from a vector with the as_slice method
    // ranges can not be obtained with indexing a vector like can be done with string
    let data = vec![1, 2, 3, 4, 5];
    let data_slice = data.as_slice();
    println!("Data slice = {:?}", data_slice);

    // slices can be converted to vectors with the .to_vec() method
    let some_slice = &[1, 2, 3, 4, 5];
    let some_vector = some_slice.to_vec();

    println!("Some vector = {:?}", some_vector);

    // always remember that  safer way to get an element from a vector is with the use
    // of the .get() method, which returns value wrapped in Some if it exist or None if it does not exist

    let first_5_nums = (1..=5).collect::<Vec<_>>();
    println!("Sixth number = {:?}", first_5_nums.get(6));

    let first_100_even_numbers: Vec<_> = (1..202).filter(|x| x % 2 == 0).collect();
    println!(
        "First 100 numbers = {:?}, len: {}",
        first_100_even_numbers,
        first_100_even_numbers.len()
    );

    // an example to convert an vector of &str types to a vector String types
    let names = vec!["james", "paul", "okon", "obot", "abasifreke"]
        .iter_mut()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    println!("Names = {:?}", names);

    // NOTE: the default iterator for a vector is consuming, to get a reference to the elements

    // intializing a vector with a certain type
    let cars = vec!["Toyota"; 10];
    println!("Cars = {:?}", cars);

    // we can append all elements from a vector to another vector and empty the appended vector onto the
    // base vetor
    let mut top_vector = vec![6, 7, 8];
    let mut base_vector = vec![1, 2, 3, 4, 5];

    base_vector.append(&mut top_vector);
    println!("Base vector = {:?}", base_vector);
    println!("Top vector = {:?}", top_vector);

    // we can insert elemetns into a vector at a certain index
    let mut data = vec![1, 2, 3, 4, 5];
    data.insert(2, 10);
    println!("Data = {:?}", data);

    // we can remove item from an index with the remove method
    let mut data = vec![1, 2, 3, 4, 5];
    let removed_item = data.remove(2);
    println!("Removed item = {}", removed_item);

    let mut first_100_numbers = (1..=100).collect::<Vec<_>>();
    // retain operates on the vector in place, this means the vector must be editable
    // as in the case for insert and remove method shown above
    first_100_numbers.retain(|x| x % 2 == 0);
    println!("First even numbers = {:?}", first_100_numbers);
}
