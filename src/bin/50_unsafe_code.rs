macro_rules! capitalize {
    ($a: expr) => {
        let mut v: Vec<char> = $a.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        $a = v.into_iter().collect();
    };
}

fn main() {
    let mut name = String::from("test");
    capitalize!(name);

    println!("{name}");
}
