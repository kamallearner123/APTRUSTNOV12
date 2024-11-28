struct Block{
    to:String,
    from:String,
    amount:u32,
    hash:String,
    next:Option<Box<Block>>
}

fn main() {
    // Allocating memory for strucuture
    let ptr = Box::new(100);

}