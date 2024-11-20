fn fun(data:&mut String) {
    println!("{}", data);
    data.push_str(" world");
}
fn main() {
    let mut data = String::from("hello");
    fun(&mut data);
    println!("{data}"); 
}