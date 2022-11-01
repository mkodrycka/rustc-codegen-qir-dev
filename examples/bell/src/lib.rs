// Minimal no_std program ripped from:
// https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html
#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
