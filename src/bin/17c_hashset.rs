use std::collections::HashSet;

fn main() {
    // hashset are collections of unique items
    let _new_sets = HashSet::<&str>::new(); // new empty hashset
                                            // hashset from an array of values
    let mut planets = HashSet::from(["Mercury", "Venus", "Earth"]);
    // use of the into method to coerce an array into a hashset
    let _another_hashset: HashSet<_> = [1, 2, 3, 4, 5].into();

    // most the similar methods available on hashmap are available here
    // let inserting a new item
    println!("Planets: {:#?}", planets);

    planets.insert("Mars");
    planets.insert("Jupiter");

    println!("Planets: {:#?}", planets);
}
