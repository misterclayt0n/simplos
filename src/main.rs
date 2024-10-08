#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
