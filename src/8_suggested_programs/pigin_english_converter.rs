fn pigin_converter(sentence: String) -> String {
    let mut pigin_form = String::new();

    for word in sentence.split_whitespace() {
        let first_letter = &word[0..1];
        let other_part  = &word[1..];
        let new_form = format!("{other_part}-{first_letter}ay ");
        pigin_form.push_str(&new_form);
    }
    pigin_form
}


fn main() {
    let sentence = String::from("Hello World from Africa");
    let pigin_form = pigin_converter(sentence);
    println!("Pigin Form = {pigin_form}");
}