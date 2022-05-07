use std::collections::HashMap;

fn main() {
    // Create & populate a new HashMap
    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);
    println!("scores1: {:?}", scores1);

    // Create & populate a new HasMap based off vectors
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores2: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("scores2: {:?}", scores2);

    // HashMaps & Ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map1 = HashMap::new();
    map1.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    println!("map1: {:?}", map1);

    // Accessing HashMap values
    let team_name = String::from("Blue");
    let score = scores1.get(&team_name);
    println!("score: {:?}", score);

    // Iterate over a HashMap
    for (key, value) in &scores1 { println!("key: {}, value: {}", key, value); }

    // Overwrite a value in a HashMap
    scores1.insert(team_name, 25);
    println!("scores1: {:?}", scores1);

    // Only insert a value in the HashMap if the key has no value yet
    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Yellow"), 6);
    scores3.entry(String::from("Yellow")).or_insert(50);
    scores3.entry(String::from("Blue")).or_insert(50);
    println!("scores3: {:?}", scores3);

    // Updating a HashMap value based on the old value
    let text = "hello world wonderful world";
    let mut map2 = HashMap::new();
    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1; // * = dereference count
    }
    println!("map2: {:?}", map2);
}
