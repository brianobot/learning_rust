// Nested Modules
mod plant_structures {
    pub mod roots {}

    pub mod stems {}

    pub mod leaves {}
}

fn main() {
    println!("Module Lessons");
    // pub(crate) makes an item visible only within the same crate
    // pub(super) makes an item visible only within the parent module
    // pub(in <path>) makes an item visible in a specific parent module
    //
    // modules can be in separate files and when compiling, they are compiled together
    // use statement causes a name to be a local alias for a path in a module
    // use std::mem::swap; // this causes the local name swap to be an alias for the path std::mem::swap;
    // it's a better style to import types, traits and modules and use relative path to access functions and their items
    use std::mem;

    let mut s1 = 10;
    let mut s2 = 20;
    mem::swap(&mut s1, &mut s2);

    println!("S1 = {s1}");
    println!("S2 = {s2}");

    // several names can be imported at once
    #[allow(unused_imports)]
    use std::collections::{HashMap, HashSet}; // 
    // shorthand for
    // use std::collections::HashMap;
    // use std::collections::HashSet;
    #[allow(unused_imports)]
    use std::fs::{self, File};
    // shorthand for
    // use std::fs;
    // use std::fs::File;
    //
    // you can use as to rename the alias for an imported item
    use std::mem as Mem;

    Mem::swap(&mut s1, &mut s2);

    // each modules starts with a blank slate and must import names it needs or uses
    // self is also synonym to the current module
    //
    mod parent {
        #[allow(dead_code)]
        pub struct Person(String);

        mod child_1 {
            // to access items in the parent module, use super::<Item>
            pub use super::Person;
        }

        // to access items in the current module, use self::<Item> or just Item
        #[allow(unused_imports)]
        use self::Person as Human;
        #[allow(unused_imports)]
        use self::child_1::Person as ChildPerson; // here i can access an item in a child module 
    }

    // the keyword super and crate have special meaning in paths
    // super refers to the parent module
    // crate refers to the crate containing the current module
    //
    // by default when you create a binary package, a binary crate with the same name as your package
    // would be automatically created and the crate root would be src/main.rs
    // a package must have atleast one crate
    //
    // you can use an absolute path to always refer to an external module as
    mod rand {
        #[allow(dead_code)]
        pub struct Person(String);
    }
    #[allow(unused_imports)]
    use ::rand::Rng; // this always points to an external crate rand even though i might have a module called rand
    #[allow(unused_imports)]
    use rand::Person; // or

    // the standard prelude is automatically included in every rust program
    use std::prelude::v1::*; // this contains a few dozen common used trait and types

    mod kitchen {
        #[allow(dead_code)]
        pub struct Cooker(pub String);
    }

    kitchen::Cooker("Pressure Cooker".to_string());
}
