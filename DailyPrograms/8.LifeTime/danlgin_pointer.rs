fn dangling<'a>() -> &'a String {
    let s = String::from("Hello");
    let ref_result: &'a String = &s;
    ref_result
}

fn main() {
    let r = dangling();
    println!("r = {}", r);
}