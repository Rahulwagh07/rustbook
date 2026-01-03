use std::collections::HashMap;

pub fn hashmap_example() {
    // create_and_insert_hashmap();
    // get_values_from_hashmap();
    // iterate_hashmap();
    // hashmap_from_vectors();
    ownership_in_hashmaps();
    entry_api_example();
    modify_value();
}

fn create_and_insert_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Scores: {scores:?}");
}

fn get_values_from_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // get() returns Option<&V>
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Score for Blue: {score}");
}

fn iterate_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, val) in &scores {
        println!("{key}: {val}");
    }
    for (key, val) in scores {
        println!("{key}: {val}");
    }
}

fn hashmap_from_vectors() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // zip produces an iterator of (team, score) tuples
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{scores:?}");
}

fn ownership_in_hashmaps() {
    let field_name = String::from("Fav color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    //moves field_name and field_value into the map
    map.insert(field_name, field_value);

    // field_name and field_value are no longer valid here
    println!("Map now has {} entries", map.len());

    //Owned values such as String are moved to the hashmap, not copied
}

// Updating or inserting only if absent (entry API)
fn entry_api_example() {
    let mut map = HashMap::new();

    map.entry(String::from("k1")).or_insert(5); //insert if missing
    map.entry(String::from("k1")).or_insert(10); //ignore key

    println!("{map:?}"); //{"k1": 5}
}

fn modify_value() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    println!("{scores:?}");
    if let Some(v) = scores.get_mut("Blue") {
        *v += 5;
    }

    println!("{scores:?}");
}
