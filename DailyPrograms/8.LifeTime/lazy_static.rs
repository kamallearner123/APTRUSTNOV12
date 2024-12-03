use lazy_static::lazy_static;

lazy_static! {
    static ref DATA: Vec<i32> = vec![1, 2, 3];
}

fn main() {
    println!("{:?}", *DATA);
}