#![allow(dead_code)]

// reference materials
// https://aloso.github.io/2021/03/28/module-system.html

// unlike in most other programming langauges
// files does not equate to modules in rust
// this means that we can have a very convulated file system of files
// but the corresponding module system for said project would be entirely different
// we need to explicitly build modules in rust and there is no implicit mapping
// between the file system and the module system yeah

// to add a file as module in our project, we must declare that file as a sub module
// for example

mod test_module; // this is called a normal module as it point a file/directory that contains the module codes

mod test_module_2 { // this is an inline module as it code are inline after the module declaration
    pub fn shout() {
        println!("Shoutttinnngggg!!!");
    }
}

// here the compiler looks for a file called test_module.rs or test_module/mod.rs 
// in the current direcory

// notice how we needed to make the function in the test_module pub (public) for it 
// to be accessible outside the module, all module contents are by default private
// out side of a module

// modules contains items, such as functions, structs, enums, variables, constants, traits
// impl blocks, extern crates, extern blocks, imports, modules, associated items

// items can be referenced with paths, paths are usually relative
// foo::bar::baz, means foo must be available in the current scope
// if we wish to start from the root module, we must start with the crate::
// super:: changes to the parent module, similar to ../ in filesystem

// imports can be used to shorted paths
// instead of foo::bar::baz evertirm we need to use baz
// we can do
// use foo::bar::baz; and then access baz directly in that scope

mod outer_space {
    mod outer_atmosphere {
        const OZONE: &str = "O3";

        mod lower_atmosphere {
            mod land {
                struct House {
                    sides: u8,
                }

                static MY_HOUSE: House = House { sides: 4 };
                // accessing an item in grandparent module
                static BREATHE: &str = super::super::OZONE;
            }
        }
    }
}

// some modules part are only accesible inside the module and they are called private
// while others are accessible outside the module and are called public

// we can use,  pub use to export a module from a different module than it was declare in

pub mod answer {
    pub const ANSWER: i32 = 42;
}

pub use answer::ANSWER;


fn main() {
    test_module::print_module();
    println!("module extended!");

    test_module_2::shout();
}