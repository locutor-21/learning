#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    use std::collections::HashMap;

    let mut scores1 = HashMap::new();

    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // Hash Maps and Ownership
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map1 = HashMap::new();
    map1.insert(field_name, field_value);
    // field_name and field_value are invalid at this point

    // Accessing Values in a Hash Map
    let team_name = String::from("Blue");
    let score1 = scores1.get(&team_name);

    for (key, value) in &scores1 {
        println!("{}: {}", key, value);
    }

    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);

    scores3.entry(String::from("Blue")).or_insert(50);
    scores3.entry(String::from("Yellow")).or_insert(50);

    println!("{:?}", scores3);
    
    // Counting letters
    let text = "hello world wonderful world";

    let mut map2 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map2);
}
