fn main() {
    // slices are contigient sequence of memory
    // slices are references to value, which are not usually a complete reference to the whole value
    // but just a subset of the initial value, so like references, they do not take ownership

    // A string slice is a reference to part of a String
    let s = String::from("Hello World People!");

    let first_word = &s[0..5]; // not the the end index is not included in the slice
    let second_word = &s[6..]; // same thing applies here

    println!("first word = {first_word}");
    println!("second word = {second_word}");

    // the starting index can be ignored as it defaults to 0
    // and so can the ending index, when the end is implied implicitly
    // so you can drop both values to get the whole string
    let len = s.len();
    let hello = &s[..5];
    let world = &s[6..];
    let word = &s[0..len];
    println!("Hello = {hello}");
    println!("World = {world}");
    println!("Word = {word}");

    let first_word = _first_word(&s);
    let second_word = _second_word(&s);
    println!("First word = {first_word}");
    println!("Second word = {second_word}");

    // string literals &str
    let s = "Hello world"; // the create an immutable string literals whose value is stored in the compiled binary
                           // this is possible because it content is known at compile time
    println!("the value of s = {s}");

    // in a nutshell
    // if you need heap allocation use String
    // if you need a different view an existing string, use &str

    // to convert from &str to String use .to_string() method
    // to convert from String to &str use & (borrowing)

    // &str == string slice
    // &String == reference to a String

    // other slices
    let a = [1, 2, 3, 4, 5, 6];

    let slice = &a[..4];

    println!("Slice = {:?}", slice);
}

fn _first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

// this function is actually very rusty, i am still learnign, cut me some slack bro
fn _second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut space_count = 0;
    let mut start_index = 0;
    let mut end_index = s.len();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            space_count += 1;
            if space_count == 1 {
                start_index = i + 1; // set the start index for the second word from the next index
            }
            if space_count == 2 {
                end_index = i;
                return &s[start_index..end_index];
            }
        }
    }

    &s[start_index..end_index]
}
