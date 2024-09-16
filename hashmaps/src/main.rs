use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("yellow"), 50);

    // accesing values in hashmaps

    let team_name = String::from("Hello");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // iterating over hashmap

    for (key, value) in &scores {
        println!("{key}: {value}")
    }

    // Updating Values

    // Overwriting a value
    scores.insert(String::from("Blue"), 25);

    // Adding a Key and Value Only If a Key Isn’t Present
    // Hash maps have a special API for this called entry that takes the key you want to check as a parameter.
    // The return value of the entry method is an enum called Entry that represents a value that might or might not exist.

    scores.entry(String::from("black")).or_insert(50);
    println!("{scores:?}");

    //Updating a Value Based on the Old Value
    let text = "Hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{{map:?}}");
}
