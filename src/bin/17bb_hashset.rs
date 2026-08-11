use std::collections::{HashMap, HashSet};

fn main() {
    let mut _ages = HashMap::<String, String>::new();
    let mut seen = HashSet::<&str>::new();

    // the insert method on hashset returns a boolean indicating whethere the item was added or not
    // the item is added when it's not already present in the hashset else is it not added
    let _inserted = seen.insert("Brian");
    let _inserted = seen.insert("Joseph");

    let _ = _ages.keys();
}
