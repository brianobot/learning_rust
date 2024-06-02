
fn main() {
    let mut names: Vec<&str> = vec!["Alpha"];

    let john = "John";
    let paul = "Paul";
    let peter = "Peter";
    let default = "Omega";

    names.push(john);
    names.push(paul);
    names.push(peter);

    println!("Names = {:?}", names);

    let first_name = names.get(1000).unwrap_or(&default);
    
    println!("First Name = {first_name}");
    
}