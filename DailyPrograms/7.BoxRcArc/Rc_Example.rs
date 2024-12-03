use std::rc::Rc;

fn main() {
    let shared_data = Rc::new(String::from("Hello, Rc!"));

    let owner1 = Rc::clone(&shared_data);
    let owner2 = Rc::clone(&shared_data);

    println!("Owner 1: {}", owner1);
    println!("Owner 2: {}", owner2);
    println!("Reference Count: {}", Rc::strong_count(&shared_data));
}
