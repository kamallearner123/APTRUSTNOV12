static mut GLOBAL: i32 = 0;

fn main() {
    std::thread::spawn(|| unsafe { GLOBAL += 1 });
    std::thread::spawn(|| unsafe { GLOBAL += 1 });
    std::time::Duration::from_millis(10000);
    unsafe {
        println!("{GLOBAL}");
    }
}