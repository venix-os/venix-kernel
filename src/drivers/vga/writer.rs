use core::fmt;

use super::colours::ColourCode;
use super::screen_buffer::{ScreenChar, Buffer, BUFFER_WIDTH, BUFFER_HEIGHT};

pub struct Writer {
    column_position: usize,
    row_position: usize,
    colour_code: ColourCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(colour_code: ColourCode, buffer: &'static mut Buffer) -> Writer {
        let mut w = Writer {
            column_position: 0,
            row_position: 0,
            colour_code: colour_code,
            buffer: buffer,
        };

        w.clear_screen();
        w
    }
    
    pub fn write_byte(&mut self, byte: u8) {
        if self.column_position >= BUFFER_WIDTH {
            self.new_line();
        }

        match byte {
            b'\n' => self.new_line(),
            byte => {
                let c = ScreenChar {
                    ascii_character: byte,
                    colour_code: self.colour_code,
                };

                self.buffer.chars[self.row_position][self.column_position].write(c);
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        self.row_position += 1;

        if self.row_position >= BUFFER_HEIGHT {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }

            self.clear_row(BUFFER_HEIGHT - 1);
            self.column_position = 0;
            self.row_position = BUFFER_HEIGHT - 1;
        }
    }
    
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20...0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            colour_code: self.colour_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn clear_screen(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }

        self.row_position = 0;
        self.column_position = 0;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
