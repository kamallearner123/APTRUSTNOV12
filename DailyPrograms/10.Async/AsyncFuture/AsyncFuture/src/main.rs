#![feature(generate_trait)]
use tokio;
use std::thread;
use std::time::Duration;

async fn fun_a() {
    thread::sleep(Duration::from_secs(2));
    println!("fun_a is done!!!!");
}

async fn fun_b() {
    thread::sleep(Duration::from_secs(2));
    println!("fun_b is done!!!!");
}
async fn fun_c() {
    thread::sleep(Duration::from_secs(2));
    println!("fun_c is done!!!!");
}
async fn fun_d() {
    thread::sleep(Duration::from_secs(2));
    println!("fun_d is done!!!!");
}
async fn fun1() -> i32 {
    println!("Calling fun_a");
    fun_a().await;
    println!("Calling fun_b");
    fun_b().await;
    println!("Calling fun_c");
    fun_c().await;
    println!("Calling fun_d");
    fun_d().await;
    0
}

#[tokio::main]
async fn main() {
    println!("Beginning!!!");
    println!("Calling fun1");
    let fut1 = fun1();
    println!("Waiting for fut1!!!");
    
    let x = loop {
        // Check: Are you still waiting for fut1 to be completed??
        if let Some(result) = fut1.try_check_completed() {
            break result;
        } else {
            fut1.try_make_progress();
            yield;
        }
    };

    println!("Waiting for fut1 is done!!! and the result is {}", x);
}
