// crate is a compilation unit in Rust Programming Language
// modules do not get compiled individually, only crates do

// a crate can be compiled into a binary or a library
// by default crate are compiled into binary 

// a crate can be compiled as library with
// rustc --crate-type=lib <file_name>.rs

// the name of the compiled library is derived from adding lib infront of the filename
// but this can customized with the --crate-name flag

pub fn public_function() -> u8 {
    12
}

fn _private_function() -> u8 {
    13
}

fn main() {}