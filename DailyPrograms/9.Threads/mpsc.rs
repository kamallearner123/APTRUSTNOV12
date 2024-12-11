use std::thread;
use std::sync::mpsc;
use std::time::Duration;


fn main() {

    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let t1 = thread::spawn(move||{
        thread::sleep(Duration::from_secs(10)); 
        println!("In thread ");
        {
            tx1.send(10).unwrap();
            println!("In thread: Received data = {}", rx2.recv().unwrap());
        }
        1000
    });
    
    loop {
        match rx1.try_recv() {
            Err(msg) => println!("Received error: try again {:?}", msg),
            Ok(data) => {println!("Data = {:?}", data); break;}
        }
        thread::sleep(Duration::from_secs(1)); 
    }
    tx2.send(10).unwrap();
    let result = t1.join();
    println!("Result = {:?}", result);

}

/*
1) Data race:
    Who does assessment
2) Locking: Manual error
    - Wrong lock
    - forget to release lock
    - fatal error when using deallocated locks
3) Sharing the data
    - 
*/


