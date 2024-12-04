use std::collections::HashMap;

fn main() {
    let mut h1 = HashMap::new();
    h1.insert("Kamal", 100);
    h1.insert("Sri", 10);
    
    // 1. Iterator
    for (key, value) in &h1 {
        println!("Key {}, Value {}", key, value);
    }

    // 2/ Deleting key from Map
    h1.remove("Kamal");
    println!("{:?}", h1);

}