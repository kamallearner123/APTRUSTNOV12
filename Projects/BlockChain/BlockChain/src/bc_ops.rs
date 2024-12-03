
use sha2::{Sha256, Digest}; // Import the Sha256 hasher
use serde_json::json;

fn get_hash(data:&str) ->&str {
  
    // Create a Sha256 object
    let mut hasher = Sha256::new();
    
    // Provide data to the hasher
    hasher.update(data);
    
    // Finalize the hash and obtain the result
    let result = hasher.finalize();
    
    // Convert the result to a hexadecimal string
    let hash_hex = format!("{:x}", result);
    
    println!("Input data: {}", data);
    println!("SHA-256 Hash: {}", hash_hex);
}


fn get_hash_2() {

    // Block data
    let index = 1;
    let previous_hash = "0000000000000000000";
    let transactions = vec!["Alice -> Bob: 50", "Bob -> Charlie: 30"];
    let timestamp = "2024-11-21T12:00:00Z";

    // Serialize block data into JSON format
    let block_data = json!({
        "index": index,
        "previous_hash": previous_hash,
        "transactions": transactions,
        "timestamp": timestamp,
    });

    // Create a Sha256 hasher
    let mut hasher = Sha256::new();
    hasher.update(block_data.to_string());

    // Finalize the hash
    let block_hash = format!("{:x}", hasher.finalize());

    println!("Block data: {}", block_data);
    println!("Block hash: {}", block_hash);

}