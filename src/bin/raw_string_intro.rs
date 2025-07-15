fn main() {
    // when a string contains a lot of double quotes, we can use the raw string syntax to escape those quotes, and backslash
    let some_raw_string = r#"This is some raw "String" "#;
    // notice how the number of # at the start must match the # at the end

    // this is also a valid raw string
    let some_other_raw_string = r##"Some other raw "String now" "##;
    let another_valid_raw_string = r###""This" is another valid raw string"###;

    println!("{some_raw_string}");
    println!("{some_other_raw_string}");
    println!("{another_valid_raw_string}");
}
