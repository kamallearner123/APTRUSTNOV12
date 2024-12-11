
async fn fetch_data() -> i32{
    0
}

// This macro sets up the main function to use the Tokio runtime, 
// which is required to execute asynchronous code in Rust.
#[tokio::main] 
async fn main() {
    println!("Hello, world!");

    let result = fetch_data().await;
    println!("Result = {}", result);
}
