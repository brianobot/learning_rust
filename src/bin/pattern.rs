fn main() {
    let slice = vec![0, 1, 0]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let joined = slice.join("");
    let int_form = i32::from_str_radix(&joined, 2).unwrap();
    println!("Int Form: {int_form}");
}
