use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let shared_data = Arc::new(Mutex::new(String::from("Hello")));

    // Clone the Arc to share with the thread
    let thread_data = Arc::clone(&shared_data);

    let handle = thread::spawn(move || {
        {
            // Lock and modify the Mutex
            let mut data = thread_data.lock().unwrap();
            data.push_str(", from the thread!");
            println!("Thread: {}", data);
            // Mutex is unlocked automatically here when `data` goes out of scope
        } // Explicit scope for clarity

        // Simulate some other work after unlocking the Mutex
        println!("Thread: Mutex is now unlocked");
    });

    {
        let mut data = shared_data.lock().unwrap();
        data.push_str(", from the main thread!");
        println!("Main: {}", data);

        // Manually unlock the Mutex
        std::mem::drop(data); // Drops the MutexGuard, unlocking the Mutex
    }

    // Wait for the thread to complete
    handle.join().unwrap();

    // Final data
    println!("Final: {}", shared_data.lock().unwrap());
}

