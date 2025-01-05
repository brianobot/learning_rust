// hashmap is not part of the default prelude, so must be imported into the crate for use
use std::collections::HashMap;
use std::collections::hash_map::RandomState as Hasher;


fn main() {
    // hashmaps can use type inference to infer the type of the keys, values in the hashmap itself
    let mut book_reviews = HashMap::new();

    book_reviews.insert(
        "Harry Potter And the Half Blood Prince".to_string(), 
        "Excellent".to_string()
    );

    // when an insertion is done in rust hashmap, the old value is returned in the insertion if the key already exist
    // in the hashmap and the new key is used to update the key
    let review = book_reviews.insert(
        "The Alchemist".to_string(), 
        "Good".to_string()
    );
    println!("First Alchemist Review: {:?}", review);

    let second_review = book_reviews.insert(
        "The Alchemist".to_string(), 
        "Excellent".to_string()
    );
    println!("Second Alchemist Review: {:?}", second_review);
    println!("_________________________");
    
    println!("Book Reviews: {:#?}", book_reviews);

    //  key-value pairs can be removed from the hashmap using the remove method
    let removed_review = book_reviews.remove("The Alchemist");
    println!("🔐🔐🔐 Removed Review: {:?}", removed_review);

    // to update the value of a key in an hashmap, just call the insert method again
    book_reviews.insert(
        "The Da Vinci Code".to_string(), 
        "Poor".to_string()
    );

    book_reviews.insert(
        "The Da Vinci Code".to_string(), 
        "Excellent".to_string()
    );

    // even though the hashmap collection stores Owned string, it can still be queried using &str
    let first_book = book_reviews.get("Harry Potter And the Half Blood Prince");
    println!("First Book Review: {:?}", first_book.unwrap());

    // hashmap with a known list of items can be initialized from an array
    let book_reviews = [
        ("Harry Potter And the Half Blood Prince", "Excellent"),
        ("The Alchemist", "Good"),
        ("The Da Vinci Code", "Poor"),
    ].iter().cloned().collect::<HashMap<&str, &str>>();
    println!("Book Reviews: {:#?}", book_reviews);

    let book_reviews_2 = HashMap::from([
        ("Harry Potter And the Half Blood Prince", "Excellent"),
        ("The Alchemist", "Good"),
        ("The Da Vinci Code", "Poor"),
    ]);
    println!("Book Reviews 2: {:#?}", book_reviews_2);

    let mut player_stats = HashMap::new();

    player_stats.entry("Strength").or_insert(100);
    player_stats.entry("Dexterity").or_insert_with(random_dexterity);

    let score = player_stats.entry("Score").or_insert(200);
    *score += 10;

    // println!("Player Stats: {:#?}", &player_stats);
    println!("Score: {}", &score);

    // below are some method available on the Hashmap collection
    // new: create a new empty hashmap
    let new_info: HashMap<&str, &str> = HashMap::new();
    println!("New Info: {:#?}", new_info);

    // with capacity: create a new hashmap with a specific capacity
    let new_map_with_cap: HashMap<String, String> = HashMap::with_capacity(10);
    println!("New Map With Capacity: {:#?}", new_map_with_cap);

    // with hashers: create a new hashmap with a specific hasher
    let mut new_map_with_hasher: HashMap<&str, &str> = HashMap::with_hasher(Hasher::new());
    println!("New Map With Hasher: {:#?}", new_map_with_hasher);

    // the capacity method returns the number of elements the hashmap can hold without reallocating
    new_map_with_hasher.insert("Some key", "Some value");
    new_map_with_hasher.insert("Another key", "Another value");
    new_map_with_hasher.insert("Yet Another", "Yet another value");
    let cap = new_map_with_hasher.capacity();
    println!("Capacity: {}", cap);

    // the keys method returns all the keys in the hashmap as an iterator in an arbitrary order
    let keys = new_map_with_hasher.keys();
    println!("________________________________");
    for key in keys {
        println!("Key: {}", key);
    }

    println!("________________________________");

    // the values method all the values in the hashmap as an iterator in an arbitrary order
    let values = new_map_with_hasher.values();
    for value in values {
        println!("Value: {}", value);
    }

    // try to iterate over an hashmap
    // it is important to point out that when iterating over a hashmap, the order of the keys is arbitrary
    let scores = HashMap::from([
        ("Player 1", 100),
        ("Player 2", 200),
        ("Player 3", 300),
    ]);
    
    let mut scores = scores;
    for (player, score) in scores.iter_mut() {
        println!("Player: {}, Score: {}", player, score);
        *score *= 2;
    }

    println!("Updated Scores = {scores:#?}");

    // len methods returns the number of elements in a the hashmap
    println!("Length: {}", scores.len());

    // is_empty method returns true if the hashmap is empty
    println!("Is Empty: {}", scores.is_empty());


}


fn random_dexterity() -> i32 {
    50
}