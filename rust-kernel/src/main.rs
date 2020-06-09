// main.rs

#![no_std] // disable standard library (explicitly linked)
#![no_main] // no access to rust runtime and crt-0, so we have to not use the normal entry point chain

use core::panic::PanicInfo; // contains the file and line of panic and panic message

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { //! means it returns never, which makes it a divergin function
    loop {}
}

#[no_mangle] // disable name mangling to ensure _start is actually used instead of compiler generated names
pub extern "C" fn _start() -> ! {
    loop {}
}

/* Completely remove main since it does not make sense without a runtime that calls it
fn main() {
    //println!("Hello, world!"); // requires the standard library
}
*/
