use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Alice", 50);
    scores.insert("Bob", 40);
    
    if let Some(&score) = scores.get("Alice") {
        println!("Alice's score: {}", score);
    }

    scores.remove("Bob");
    println!("{:?}", scores); // {"Alice": 50}
}

