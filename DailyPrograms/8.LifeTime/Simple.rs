fn main() {
    let r;
    {
        let x = 42;
        r = &x;
    } 
    println!("r: {}", r);
}