#![no_std]

use core::panic::PanicInfo;

extern crate rlibc;
extern crate spin;
extern crate lazy_static;
extern crate volatile;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {
    println!("Hello World!");
    loop { }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop { }
}
