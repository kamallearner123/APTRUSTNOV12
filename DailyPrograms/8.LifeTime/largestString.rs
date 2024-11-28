fn main() {
    let a = "hello";
    let result; // a
    {
        let b:&str = "world"; // a, b
        let c:&str = "word c";
        result = get_largest(&a, &b, &c); // a,b
    }
    println!("Largest string = {}", result); //a, b
}

// life time of a : 8
// life time of b : 8

fn get_largest<'a_lifetime>(a:&'a_lifetime str, b:&'a_lifetime str, c:&'a_lifetime str) -> &'a_lifetime str {
    if a.len() > b.len() {
        a // 'a
    } else if a.len() == b.len(){
        c // 'b
    } else {
        b
    }
}