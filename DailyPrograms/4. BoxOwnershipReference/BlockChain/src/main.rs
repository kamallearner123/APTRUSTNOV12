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

// impl Drop for Block {

// }

fn main() {
    // let ptr: Box<i32> = Box::new(int{10}); // Allocates memory for integer
    // // int *ptr = malloc(sizeof(int));
    // // *ptr 
    // println!("Value = {}", *ptr);

    let mut block1: Block = Block::new(String::from("Moh"), 
                                    String::from("Bitcoin"),
                                    100,
                                    String::from("00"),
                                    None);
    let first_block = Box::new(&block1);
    println!("To={}", first_block.to);

    block1.to = String::from("Sri");
    let second_block = Box::new(&block1);
    println!("To={}", second_block.to);

    
}
