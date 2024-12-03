
#[derive(Debug)]
enum Grade {
    FIRST,
    SECOND,
    THIRD,
    FAIL
}
#[derive(Debug)]
enum ErrorCode {
    OUTBOUND,
    INVALID
}

fn get_grade(marks: i8) -> Result<Grade, ErrorCode> {
    match marks {
        60..=100 => Ok(Grade::FIRST),
        60..=79 => Ok(Grade::SECOND),
        _ => Err(ErrorCode::INVALID)
    }
}

// fn get_grade(marks: i8) -> Result<Grade, ErrorCode> {
//     if marks > 100 {
//         Err(ErrorCode::OUTBOUND)
//     } else if marks>80 {
//         Ok(Grade::FIRST)
//     } else if marks>60 {
//         Ok(Grade::SECOND)
//     } else if marks>40 {
//         Ok(Grade::THIRD)
//     } else if marks>=0{
//         Ok(Grade::FAIL)
//     } else {
//         Err(ErrorCode::INVALID)
//     }
// }



fn sample(marks: i8) -> Result<Grade, ErrorCode> {

    let result = get_grade(marks)?;

    println!("After ?");
    Ok(result)
}

fn main() {
    let marks = -1;
    // let result = match get_grade(marks) {
    //     Ok(rank) => ErrorCode::OUTBOUND,
    //     Err(error) => {
    //         match error {
    //             ErrorCode::OUTBOUND => {println!("outbound: correct the input"); ErrorCode::OUTBOUND},
    //             ErrorCode::INVALID => {println!("invalid: check it!!!"); ErrorCode::OUTBOUND}
    //         }
    //     }
    // };
    // let result = if marks > 80 {
    //     Grade::FIRST
    // } else {
    //     Grade::SECOND
    // };
    println!("{:?}", get_grade(60));
}