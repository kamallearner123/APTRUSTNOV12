static mut GLOBAL: i32 = 0;

fn main() {
    std::thread::spawn(|| unsafe { GLOBAL += 1 });
    std::thread::spawn(|| unsafe { GLOBAL += 1 });
    unsafe {
        println!("{GLOBAL}");
    }
}
