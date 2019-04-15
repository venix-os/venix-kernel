#![cfg_attr(not(test), no_std)]
#![cfg_attr(test, allow(unused_imports))]

#[cfg(test)]
extern crate core;

use core::panic::PanicInfo;

extern crate spin;
extern crate lazy_static;
extern crate volatile;

#[macro_use]
mod drivers;

#[no_mangle]
pub extern fn rust_main() -> ! {
    println!("Hello World!");
    loop { }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop { }
}
