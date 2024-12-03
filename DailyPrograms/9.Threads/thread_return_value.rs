use std::thread;
use std::time::Duration;
use std::sync::{Arc,Mutex};
/*
Option:
    Some(100)
    None

Result:
    Ok(100)
    Err("msg")
*/


fn main() {
    let args = vec!["1","2"];
    let mut count = Arc::new(Mutex::new(0)); 
    let mut tids = Vec::new();
    for arg in args{
        let mut ref_count = Arc::clone(&count); // creating reference
        let tid = thread::spawn(
            move || {
                thread::sleep(Duration::from_secs(1)); 
                println!("Thread: Thread created!!!");//2
                println!("Version: {:?}, arg = {:?}", ref_count, arg); // Trying to borrow from line 7
                {
                    let mut ptr_count = ref_count.lock().unwrap();
                    *ptr_count += 1;
                }// ref_count will be unlocked

            }
        );

        tids.push(tid);
    }

    for tid in tids {
        tid.join();
    }
    //tid: thread::JoinHandle
    let mut ptr_count = count.lock().unwrap();
    println!("Total Count = {}", *ptr_count);
}
