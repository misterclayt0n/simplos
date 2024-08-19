use core::panic::PanicInfo;

#![no_std]

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() { }
