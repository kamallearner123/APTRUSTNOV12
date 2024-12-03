use std::collections::BTreeMap;

fn main() {
    let mut t1 = BTreeMap::new();
    t1.insert("Kamal", vec![1,2,3]);
    t1.insert("David", vec![1,2,3]);
    t1.insert("Azad", vec![1,2,3]);
    t1.insert("Sri", vec![1,2,3]);


    for (key, value) in t1 {
        println!("Key: {}, Value: {:?}", key, value);
    }
}

