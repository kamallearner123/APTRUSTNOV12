#[derive(Debug)]
struct Student <'a>{
    name:&'a str,
}

fn create_instance<'a>() -> Box<Student<'a>> {
    let iname = "Kamal";
    Box::new(Student{name:&iname})
}

fn main() {
    println!("Instance = {:?}", create_instance());
}