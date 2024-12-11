
// cargo-deps: tokio = "1.0"

use std::future::Future;

#[tokio::main]
async fn main() {
    get_data().await; // Await the `get_data` async function
}

async fn get_data() {
    println!("In: get_data");
}
