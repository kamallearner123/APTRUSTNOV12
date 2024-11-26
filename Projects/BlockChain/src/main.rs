/*
1) Structure
    - Defining structure
    - Creating instance
    - Writing member function
2) Box: malloc
    - How to allocate memory
    - Idea on how the memory is deallocated
3) Enum : Option
    - Defining Enums
    - Using "match"
4) Exmaple  to implement Linked List (Block chain)
    - Creating blocks
    - Linking

5) Create a https server
    - Create a server
    - Handle request
        - Parse the request, verify the transaction
        - If valid, add to the block chain
    - Send response

6) 
*/

use std::thread;

mod https_handler;

/*
Block Chain
    - Block
        - to
        - from
        - amount
        - hash
        - next
*/
struct Block {
    to:String,
    from:String,
    amount:u32,
    hash:String,
    next:Option<Box<Block>>
}

impl Block {
    fn new(ito:String,
            ifrom: String,
            iamount: u32,
            ihash: String,
            inext: Option<Box<Block>>) -> Self {
        Block{
            to:ito,
            from:ifrom,
            amount:iamount,
            hash:ihash,
            next: None
        }
    }
}



fn main() {

    println!("{}",https_handler::get_version());
    let https_tid: thread::JoinHandle<()> = https_handler::create_https_server_thread();
    println!("HTTPS Server started with thread id = {:?}", https_tid);

    https_tid.join().unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    // let mut block1: Block = Block::new(String::from("Moh"), 
    //                                 String::from("Bitcoin"),
    //                                 100,
    //                                 String::from("00"),
    //                                 None);
    // let first_block = Box::new(&block1);
    // println!("To={}", first_block.to);

    // block1.to = String::from("Sri");
    // let second_block = Box::new(&block1);
    // println!("To={}", second_block.to);

    
}
