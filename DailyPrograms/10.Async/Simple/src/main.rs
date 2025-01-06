use tokio::time::{Duration, sleep};

async fn fetch_task(command: String) -> String {
    println!("Started task {}", command);
    sleep(Duration::from_secs(2)).await;
    format!("Command = {command}\n")
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");


    // let (r1,r2,r3) = tokio::join!( fetch_task("Command1".to_string()),
    //                                 fetch_task("Command2".to_string()),
    //                                 fetch_task("Command3".to_string()));

    let r1 = fetch_task("Command1".to_string()); // r1 : Future
    println!("Called asyn function!!!");
    r1.await;

    //println!("Result = {:?}", r1);
}