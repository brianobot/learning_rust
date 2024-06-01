use std::collections::HashMap; // since this is the least use collection, it is not included in the prelude

fn main() {
    // HASHMAPS ~ less complicated than strings they say
    // they are dictionaries in python, key value pair which allow the keys to be used to access to values
    // they keys can be anything? 

    // just like with other collections types , when creating an empty collection the type must be annotated
    // but again if this values are provide
    let mut scores: HashMap<String, i32> = HashMap::new();

    // there is no built in macro to construct hashmaps
    // the hashmap data is store on the heap like vectors too,
    // all keys must be of the same type and all values must be of the same type

    println!("Hashmap = {:?}", scores);

    scores.insert(String::from("Brian"), 10);
    scores.insert(String::from("Joseph"), 10);
    scores.insert(String::from("Anthony"), 10);
    scores.insert(String::from("Emediong"), 10);

    println!("Score = {:?}", scores);

    let name = String::from("Brian");
    // get returns an Option<&V>, we used to get the owned version Option<V>
    // we then use unwrap_or to get the value of a default if the value is None 
    let score = scores.get(&name).copied().unwrap_or(0);

    let value = None.unwrap_or(0);

    println!("{name}'s Score = {score}");
    println!("Value = {value}");

    // iterating over an hashmap
    println!("===================================");
    println!("SCORES");
    println!("===================================");
    for (key, value) in &scores {
        let key_len = key.len();
        let space = 33 - key_len;
        println!("{key}: {:space$}", value);
    }
    println!("===================================");

    println!("Scores = {:?}", scores);
    // interesting facts also, there is no order in the keys stored in the hashmaps, just like in python
    

}