#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _; // Halt execution on panic

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();

    // Configure the LED pin (PB5 = pin 13) as output
    let mut led = dp.PORTB.pb5.into_output();

    // Blink the LED in an infinite loop
    loop {
        led.toggle(); // Toggle the LED state
        arduino_hal::delay_ms(1000); // Delay for 1 second
    }
}
