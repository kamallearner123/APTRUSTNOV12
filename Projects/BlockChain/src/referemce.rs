
static mut msg:String = String::from("something");

fn fun<'a>(input: &'a str) {
    println!("Input = {}", input);

    msg = input;
}

fn main() {
    let s1: &str = "Hello";
    fun(&s1);

}