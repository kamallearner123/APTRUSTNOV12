use std::io;

fn sample(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?; 
    Ok(content)
}

fn main() {
    // String -- Get copied in heap
    // &str -- Gets copied in stack
    let result = sample("./SimpleResult1.rs");
    let rus = result.unwrap();
}
