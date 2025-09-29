use std::collections::HashMap;

fn main() {
    // Their are like dictionaries in python.
    // Like vectors, all the keys and values must be the same value type
    let mut scores = HashMap::new();

    // If a value or key is moved to HashMap, this one takes ownership.
    scores.insert(String::from("Yellow"), 10);
    scores.insert(String::from("Blue"), 50);

    // To get a value from
    let team_name = String::from("Blue");
    // We get the value as Option<&i32>, with copied() we convert to Option<i32>
    // and with unwrap, if None with set as value 0.
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // As vectors, we can iterate 
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // To update, overwriting a value
    scores.insert(team_name, 25);
    println!("{scores:?}");

    // A common practice is check if the key exist, this is checked with entry
    // Thi return a Entry enum if exists or not
    scores.entry(String::from("Blue")).or_insert(50);
    // or_insert returns the mutable reference from Entry if exists or assign a value to
    // the new key
    scores.entry(String::from("Orange")).or_insert(50);
    println!("{scores:?}");

    // To update a value based in old one
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // Add the word with count 0 or return reference to count value
        let count = map.entry(word).or_insert(0);
        // Uodate value
        *count += 1;
    }
    println!("{map:?}");
}