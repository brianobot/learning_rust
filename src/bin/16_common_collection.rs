#[allow(unused)]

fn main() {
    // rust standard libraries contain a number of useful data structures called collections
    // collections can contain multiple values
    // the data that collection point to is store in the heap, unlike the array and the tuple
    // since collections values are stored on the heap, the amount of data do not need to be known at compile time

    // the three most commonly used collections in rust are 
    // vector - similar to list in python 
    // string - collection of characters
    // hashmap - key value pair similar to dictionary in python

    // Vector Vec<T>
    // they can only store values of the same type

    let vec: Vec<i32> = Vec::new(); // type annotation is needed for the empty state
    // if instead we were creating the vector with some initial values, we would not have to specify the 
    // type since rust can infer the type from the values we give to the vector

    let v = vec![1, 2, 3, 4]; // rust programs a vec macro that can create a vector with the values we give to it

    // just like any other variable in rust, if we need to change the value of the vector
    // we need to make it mutable

    let mut vector = vec![]; // this also creates a new empty vector too
    // the compiler knew from the data added to the vector, that the support data type for the vector is i32

    vector.push(5);
    vector.push(4);
    vector.push(3);
    vector.push(2);
    vector.push(1);
    vector.push(0);

    println!("Vector = {:?}", vector);

    // reaading elements of vectors
    let mut points = vec![12, 23, 45, 67, 87, 89];

    let first_point = points[0];
    let second_point = points[1];

    println!("First point = {first_point}");
    println!("Second point = {second_point}");

    // we can also use the get method to read element from the vector
    // when we use & and [] we get a reference to the element at the index provided

    let ref_first_element = &points[0];
    println!("Reference to the first element = {ref_first_element}");

    // when we use the get method we get a Option, which would contain a reference element in the 
    // particular index if it exists or None
    let tenth_element = points.get(10);
    match tenth_element {
        Some(value) => println!("Tenth element is {value}"),
        None => println!("There is no tenth element"),
    }

    // if we hold a reference to an element in the vector and try to add to it 
    // this would not work because of the rule of not having any other references when you have a mutable reference

    // points.push(100);
    // println!("Points = {:?}", points);

    println!("Reference to the first element  = {ref_first_element}");
    // this is because the compiler has seen that between the time of creating the immutable reference the 
    // object actually mutates, this does not completely make sense to me but i accept it logic

    // okay the explanation for this is that, since vector store it items next to each other, when a new item is added
    // if the remain space is not enough to add the item next to the other items, the while vector is moved to a new memory space
    //  in such case, any reference to the former memory region would be invalid, hence rust prevents this altogether

    // we can iterate over the elements in a vector in turn in a for loop
    for point in points {
        println!("Point = {}", point);
    }

    let mut coord = vec![12, 23, 43, 56, 67, 78, 98, 54, 43];

    for i in &mut coord {
        *i += 50;  // we use the * to dereference the element here
    }
    println!("Coord = {:?}", coord);

    // we can stimulate a scenario where a vector hold different data types using an enum
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(12),
        SpreadsheetCell::Float(12.0),
        SpreadsheetCell::Text(String::from("Some information")),
    ];

    let sheet = vec![
        &row,
        &row,
    ];

    println!("Row = {:?}", row);
    println!("Sheet = {:?}", sheet);

    // String
    // Rust has only one string type in the core language, which is the string slice str
    // String type is not coded into the core language but provided by a standard library
    // String is actually implemented as a wrapper around a vector of bytes with some extra guarantees,

    // creating a new String
    let s = String::new();

    // if we have some initial data we can use the to_string method to convert a str to a String
    let data = "Hello World";

    // the method also works on a literal directly:
    let s = data.to_string();

    println!("s = {s}");
    
    let s = String::from("Initial data goes here");

    println!("s = {s}");
    // Strings can grow in size and its content can change,if you push more data to it

    let mut name = String::from("Brian");
    name.push_str(" David Obot"); // this method takes string slice since it does not need to take ownership
    // of the string passed to it

    // just like the push_str, the push method takes a single character and extends the string by that character
    name.push('.');
    name.push('💞');

    // the + operator can be used to grow a string too and the format macro

    println!("Name = {name}");

    // string concatenation with + operator
    let s1 = "Hello ".to_string();
    let s2 = "World".to_string();
    let new = s1 + &s2; // after this line, s1 has been moved
    // the compiler can coerce &String argument into &str
    println!("New = {new}");
    println!("S2 = {s2}");

    // when adding multiple strings together it is better to use the format macro
    // it is easier and does not take ownership of the strings passed to it
    let s1 = "Hello ".to_string();
    let s2 = "World!".to_string();
    let s = format!("{s1} {s2}");

    println!("S = {s}");
    
    // NOTE: rust does not support String indexing as seen in other programming languages
    // the simple explanation is that in rust, a byte does not always correspond to a character 
    // since some characters can be reprensented by more than 1 byte, which would make indexing fail in some cases

    let hello = String::from("Здравствуйте"); // this is a valid string with 12 characters
    // but each character takes 2 bytes each which amount to 24 bytes, now this code below would fail

    // let first = &hello[0];
    // so rather than user the [] with a specific number as the index, rust encourages the use of a range to
    // get the slice of string

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("S = {s}");

    // if the boundary of the slice is inbetween a character, rust would panic, 
    // some characters take up more than 1 byte remember!!
    // let f = &hello[0..1];

    // iterating over a string
    // first method is iteracting over each character

    for i in hello.chars() {
        println!("c = {i}");
    }

    // second method is iterating over each byte
    for i in hello.bytes() {
        println!("byte = {i}");
    }

    // But be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.


}