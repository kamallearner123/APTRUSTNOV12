
static mut msg:String = String::from("something");

fn fun<'a>(input: &'a str) {
    println!("Input = {}", input);

    msg = input;
}

fn main() {
    let s1: &str = "Hello";
    fun(&s1);

}


// Write test cases for fun() function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fun() {
        let s1: &str = "Hello";
        fun(&s1);
        unsafe {
            assert_eq!(msg, "Hello");
        }
    }
}
