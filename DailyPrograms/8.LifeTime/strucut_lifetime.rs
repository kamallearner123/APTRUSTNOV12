
// // #[derive(Debug)]
// // struct Block <'a> {
// //     to: &'a str,
// // }

// // impl<'a> Drop for Block<'a> {
// //     fn drop(&mut self) {
// //         println!("Dropping instance!!!");
// //     }
// // }
// #[derive(Debug)]
// struct content;
// impl Drop for content{
//     fn drop(&mut self) {
//         println!("Drop of content is called!!");
//     }
// } 

// #[derive(Debug)]
// struct Block <'a>{
//     to: &'a str,
//     con: &'a content,
// }

// impl<'a> Drop for Block<'a> {
//     fn drop(&mut self) {
//         println!("Dropping instance!!!");
//     }
// }


// fn main() {
//     let con1 = content{};
//     let ob1;
//     {
//         let s1 = "hello";//Readonly memory
//         //let con1 = content{};
//         ob1 = Block{to:&s1, con:&con1};
//         println!("Exiting from closure.....");

//     }
//     println!("Exited from closure********");
//     println!("ob1 = {:?}", ob1);
// }


fn dummy() ->&'static str {
    let comment: &'static str = "Commen1";
    return comment;
}

fn main() {
    let result = dummy();
    println!("result = {}", result);
}