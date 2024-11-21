fn main() {
    // 1. Allocates memory for integer
    let mut ptr = Box::new(String::from("It is Rust!!!");
    // 2. Accessing reference
    println!("Data in ptr = {}", *ptr); 
    //3. Modifying the value
    *ptr += 20;
    println!("Data in ptr = {}", *ptr);
    // Automatically gets deallocated
}