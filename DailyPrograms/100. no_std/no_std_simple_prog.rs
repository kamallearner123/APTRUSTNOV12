#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn paninc(_info: PanicInfo) ->! {
    loop {}
}


#[no_mangle]
pub extern "C" _start () -> ! {
    loop {}
}
