fn main() {
    // arrays, tuples, slice, slice string

    // arrays must contain homogenous datatype
    // arrays are fixed length, once created can be mutated if defined with mut
    // but the size can not be changed

    // rust arrays are different from c arrays, they are stored on the stack
    // and there is no pointer arithmetic needed for accessing the array items
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    let _other_numbers = [2u8, 3, 4, 5, 2];
    // i declared the first element to be a u8 and rust knows that an array is homogenous
    // so it makes the other elements u8 too, this could have also be achieved using a type annotation
    let _other_other_numbers: [u32; 5] = [1, 2, 3, 4, 5];

    let fruits = [
        String::from("apple"),
        String::from("orange"),
        String::from("paw-paw"),
    ];
    let vegetables: [&str; 3] = ["carrot", "cucumber", "tomatoe"];
    let human = ("Alice", 30, false);
    let more_mature_tuple = ("Okon", 23, true, [1, 2, 3, 4, 5]);
    let number_slice = &[1, 2, 3, 4, 5];

    println!("FRUITS = {:?}", fruits);
    println!("Vegetables = {:?}", vegetables);
    println!("Human Tuple = {:?}", human);
    println!("More mature tuple = {:?}", more_mature_tuple);
    println!("Number slice = {:?}", number_slice);

    // you can do a default initialization as so
    let mut zeros = [0; 5];

    numbers[0] = 0;
    // notice how the spaces inside the square bracket does not matter
    zeros[zeros.len() - 1] = 1;

    // tuples can contain hetergeneous datatypes
    let data = (1, 2, 3, true, "apple");

    println!("Numbers array = {:?}", numbers);
    println!("Zeros = {:?}", zeros);
    println!("Data = {:?}", data);

    // vectors created with ::new or vec! are the same
    // and unless something is put into a vector, it is not allocated
    // which means a vector withour element does not consume memory

    // we can annotate a vector and tell rust to infer the type of the items in the Vector
    let some_data: Vec<_> = vec![1, 2, 3, 4, 5, 6];
    println!("Some data = {:?}", some_data);
}
