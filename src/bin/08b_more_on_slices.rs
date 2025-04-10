use std::mem;

fn main() {
    // slices have the type signature &[T]

    fn analyze_slice(slice: &[i32]) {
        println!("The first element of the slice is {}", slice[0]);
        println!("The slice has {} elements", slice.len());
    }

    let arr = [1, 2, 3, 4, 5, 6];
    analyze_slice(&arr);

    // all elements in an array can be initialized at the same time
    let zeros = [0; 500];
    println!("Zeros = {:?}", zeros);

    // indexing for slice and arrays works the same way
    let first_array_element = zeros[0];
    let first_slice_element = &zeros[0];

    println!("First array element = {}", first_array_element);
    println!("First Slice element = {}", first_slice_element);

    println!("Array occupies {} bytes", mem::size_of_val(&arr));

    // remember slice can point to a section of an array
    let last_hundred_zeros = &zeros[400..];
    println!("Last hundred zeros = {:?}", last_hundred_zeros);
    println!(
        "Count for the last hundred zeros = {}",
        last_hundred_zeros.len()
    );

    // slice can be empty
    // when defining empty arrays, the type annotation must be provided since it can not be inferred from the definition
    let empty_array: [i32; 0] = [];
    println!("Empty array = {:?}", empty_array);
    let empty_slice = &empty_array;
    println!("Empty slice = {:?}", empty_slice);

    // array element can be safely accessed with get method, this returns an Option
    // and can be matched as showned below
    let fifth_element = empty_array.get(5);
    let value = match fifth_element {
        Some(value) => value,
        // since the value of value in the Some variant was a reference to an i32
        None => &0,
        // i have to borrow here to ensure the return value here matches that case of the Some variant
    };

    println!("Value = {}", value);
}
