use std::collections::HashMap;

fn main() {
    // 1. Create a variable
    let mut h1 = HashMap::new();

    // 2. Insert: Key, Value pair.
    h1.insert("Kamal", 100);
    h1.insert("Sri", 10);
    // 3. Updating
    h1.insert("Kamal", 10); // Updating existing one.
    
    // 4. Accessing
    println!("Poped pair : {:?}", h1.get("Kamal"));

}