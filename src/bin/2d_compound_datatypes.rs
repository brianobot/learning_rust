#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

// again unlike methods in other languages 
// methods in rust are implemented in an impl block and not in the actual struct block

impl File {
    fn new(name: String) -> File {
        File {
            name,
            data: vec![],
        }
    }
}


fn main() {
    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };

    let f2 = File::new(String::from("new_file.txt"));

    let f1_name = &f1.name;
    let f1_data = &f1.data;

    let f2_name = &f2.name;
    let f2_name = &f2.data;

    println!("f1 = {:?}", f1);
    println!("F1 name = {}", f1_name);
    println!("F1 data = {:?}", f1_data);
}
