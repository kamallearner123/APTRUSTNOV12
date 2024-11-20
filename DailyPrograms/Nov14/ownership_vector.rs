
#[derive(Debug, Copy, Clone)]
struct my_data{
    val1:i32,
    val2:i32,
    val3:String
}


fn main() {
    let a:my_data = my_data{val1:1, val2: 100, val3:String::from("hi")};
    println!("{:?}",a);

    let b = a;//?Yes
    println!("{:?}",a); //Yes
}