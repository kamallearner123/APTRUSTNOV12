use std::io;

fn sample(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}

fn main() {
    let result = sample("./return_error1.rs");
    println!("{:?}", result);
}

