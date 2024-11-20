fn take_ownership(data:String) {

}

fn main() {
    let s1 = String::from("hello");
    let s = s1.clone();
    take_ownership(s1);
    println!("s ={}",s);
}