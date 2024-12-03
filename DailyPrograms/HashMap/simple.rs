use std::collections::HashMap;

fn main() {
    let mut map1 = HashMap::new();

    map1.insert("Bats Men", vec!["Kohli","Surya"]);
    map1.insert("Bowlers", vec!["Bhumra", "Kapil"]);
    map1.insert("team", vec!["Bhumra", "Kapil"]);
    map1.insert("crew", vec!["Bhumra", "Kapil"]);
    map1.insert("helpers", vec!["Bhumra", "Kapil"]);
    
    for (key, value) in map1 {
        println!("Key ={}, Value = {:?}", key, value);
    }
}
