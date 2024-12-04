
// Statement: Find the frequency of words in given text
// Clue: Use HashMap
use std::collections::HashMap;
fn main() {
    let statement = "Hello world, work is devine";
    let mut freq = HashMap::new(); // 1) Empty HashMap

    for word in statement.split_whitespace() {
        println!("word = {}", word);
        let mut count = freq.entry(word).or_insert(0); // 2) Get mutable reference of value
        *count = *count+1; // 3) Increment value
    }

    println!("Freq = {:?}", freq);
}

