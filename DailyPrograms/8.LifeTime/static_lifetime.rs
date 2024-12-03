fn return_static(s1:&[i32]) ->&[i32] {
    return &s1[5..]
}


fn main() {
    let arr = [1,2,3,4,5,6,7];
    let data = return_static(&arr);
    println!("{:?}",data);
}



// Simple Example//
// fn return_static(s1:&str) ->&str {
//     return &s1[5..]
// }


// fn main() {
//     let data = return_static("kamal kumar mukiri");
//     println!("{}",data);
// }

// fn longest_string<'static, 'a>(s1:&'a str, s2:&'a str) -> &'static str{
//     let result = if (s1.len() > s2.len()) {
//         //return s1; // Stop and return from function
//         s1 // Result of if condition block
//     } else {
//         s2
//     };
//     println!("result = {}", result);
//     result
// }

// fn dummy_function() {
//     let result;
//     let a:&str = "hello"; //&str
//     {
//         let b = "world!!!";
//         result = longest_string(&a, &b);
//     }
//     println!("result = {}", result);
// }

// fn main() {
//     dummy_function();
// }

