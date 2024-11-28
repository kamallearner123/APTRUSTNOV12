#[derive(Debug)]
struct Block{
    to:String,
    from:String,
    amount:u32,
    hash:String,
    next:Option<Box<Block>>
}

impl Block {
    fn new(from_person:String, to_person:String, amount_paid:u32) ->  Block {
        Block{to:to_person,
            from:from_person,
            amount:amount_paid,
            hash:String::from("00"),
            next:None}
    }
}


impl Drop for Block {
    fn drop(&mut self) {
        println!("Going to be deallocated!!!!");
    }
}

fn create_block(from_person:String, to_person:String, amount:u32) -> Box<Block> {
    let block1: Box<Block> = Box::new(Block::new(from_person, 
        to_person,
        amount));

    block1
}

fn main() {
    let ptr = Box::new(String::from("Hello")); 
    /*
    Stack :
        ptr
    Heap:
        s1.ptr, s1.capacity, s1.len 
        "Hello"    
     */

    let s1 = String::from("Hello");
    /*
    Stack :
        s1.ptr, s1.capacity, s1.len 
    
    Heap:
        "Hello"    
     */


    let b1 = Box::new(Block::new(String::from("Kamal"), 
                String::from("Mohammed"), 100));
    
    println!("b1 = {:?}", b1);

    //Converting to raw pointer//
    let ptr = Box::leak(b1); // static 'a

    println!("ptr = {:?}", ptr.to);

    unsafe {
        // Regain owndership back
        let box1 = Box::from_raw(ptr);
        let box2 = Box::from_raw(ptr);
        println!("box1 = {:?}", box1);
    }


}


