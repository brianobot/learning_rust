#![allow(unused_imports)]
use rand::Error; // i do not need this, i just needed an excuse to use the attribute above

#[allow(dead_code)]
#[derive(Default, Debug)]
struct S {
    field_1: i32,
    field_2: i32,
    field_3: String,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Titan {
    Brian,
    Elon,
    Einstein,
}

#[allow(clippy::derivable_impls)]
impl Default for Titan {
    fn default() -> Self {
        Titan::Brian
    }
}

impl Titan {
    fn new() -> Self {
        Titan::Brian
    }

    fn upgrade(&mut self) {
        *self = match self {
            Titan::Brian => Titan::Elon,
            Titan::Elon => Titan::Einstein,
            Titan::Einstein => Titan::Brian,
        }
    }
}

impl S {
    fn new() -> Self {
        S {
            ..Default::default()
        }
    }
}

fn main() {
    let s = S::new();
    println!("S = {:?}", s);

    // more on default values
    let default_i32: i32 = Default::default();
    let default_string: String = Default::default();
    let default_str: &str = Default::default();
    let default_titan: Titan = Default::default();
    let default_option: Option<i32> = Default::default();
    // let default_result: Result<i32, i32> = Default::default();
    println!("Default i32 = {}", default_i32);
    println!("Default String = {default_string}");
    println!("Default Str = {}", default_str);
    println!("Default Titan = {:?}", default_titan);
    println!("Default Option = {:?}", default_option);

    let mut titan = Titan::new();
    titan.upgrade();

    println!("Titan = {:?}", titan);
}
