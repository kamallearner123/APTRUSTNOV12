use std::thread;
use std::sync::{Arc,Mutex};

// Write function to add numbers


fn main() {
    /*
        Mutable/Immutable : mut
        shared across threads (reference): Arc
        allow threads to modify the data: Mutex 
     */

    let mut var:Arc<Mutex<String>> = Arc::new(Mutex::new(String::from("Hello"))); //move    
    let mut aref_var = Arc::clone(&var); //Creating a ref pointing to Hello

    let t1 = thread::spawn(move||{
        println!("In thread {:?}", aref_var);
        {
            let mut ptr = aref_var.lock().unwrap();
            ptr.push_str("slave thread");
        }
    });

    {
        let mut ptr1 = var.lock().unwrap(); // locked
        ptr1.push_str("By main thread");
        println!("String = {}", ptr1);    
    }

    t1.join();


}
