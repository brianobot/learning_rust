mod house {
    const _HOUSE_NUMBER: u32 = 100;
    pub mod bedroom_1 {
        pub fn is_light_on() -> bool {
            true
        }
    }

    pub mod bedroom_2 {
        pub fn is_light_on() -> bool {
            false
        }
    }
}


fn main() {
    // modules are a way to organise our codes in a project
    // Module Cheat Sheet
    // - start from the crate root to compile a crate
    // - modules can be declared in the crate root
    //   - if you declare a module as mod garden
    //   - the compiler would look for modules in the following place
    //      - inline at the point of the declarationw within block
    //      - In the file src/garden.rs
    //      - In the file src/garden/mod.rs

    println!("House module example");

    // access the module functions
    println!("House one light: {}", house::bedroom_1::is_light_on());
    println!("House two light: {}", house::bedroom_2::is_light_on());
}