fn main() {
    let mut s = String::from("ఎలా ఉన్నావు ప్రియతమా");
    println!("String = {}", s);
    for i in s.bytes() {
        println!("Bytes = {}", i);
    }

    for letter in s.chars() {
        print!("{letter}");
    }
    println!("Len = {}", s.len());
}