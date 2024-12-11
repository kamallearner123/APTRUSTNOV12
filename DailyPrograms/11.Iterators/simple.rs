fn main() {
    let mut v1 = vec![1,2,3,4];

    // for num in v1.iter_mut() {
    //     println!("num = {}", num);
    //     *num = *num + 100;
    // }
    // println!("v1 = {:?}", v1);

//     v1.iter_mut().for_each(|x| (*x)*=2);
//     println!("v1 = {:?}", v1);

    let map_result : Vec<_> = v1.iter().map(|x| x+200).collect();

    let filter_result: Vec<_> = v1.iter().filter(|x| *x%4==0).collect();

    //let fold_result: Vec<_> = v1.iter().fold(|x,y| x+y).collect();
    println!("fold_result = {:?}", fold_result);    
}