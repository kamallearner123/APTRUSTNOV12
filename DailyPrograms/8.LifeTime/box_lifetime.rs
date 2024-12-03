#[derive(Debug)]
struct Block {
    param1:i32,
}

impl Drop for Block {
    fn drop(&mut self) {
        println!("Dropping!!!");
    }
}

fn dummy <'a>() -> &'a str{
    let message = "hello";
    &message
}

fn dummy_main<'a>() ->&'a str {
    let result = dummy();
    return result;
}

fn main() {
    let result = dummy_main();
    println!("result = {}", result);
}