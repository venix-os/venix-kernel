use core::fmt;
use spin::Mutex;
use lazy_static::lazy_static;

mod colours;
mod screen_buffer;
mod writer;

pub use self::colours::Colour;
use self::colours::ColourCode;
use self::screen_buffer::Buffer;
use self::writer::Writer;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(
        ColourCode::new(Colour::White, Colour::Black, false),
        unsafe { &mut *(0xb8000 as *mut Buffer) },
    ));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::drivers::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
