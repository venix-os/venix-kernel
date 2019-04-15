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

#[cfg(test)]
mod test {
    extern crate array_init;
    
    use super::*;
    use super::super::colours::Colour;
    use volatile::Volatile;

    fn construct_writer() -> Writer {
        use std::boxed::Box;

        let buffer = construct_buffer();
        Writer::new(ColourCode::new(Colour::Blue, Colour::Magenta, false), Box::leak(Box::new(buffer)))
    }

    fn construct_buffer() -> Buffer {
        use self::array_init::array_init;
        
        Buffer {
            chars: array_init(|_| array_init(|_| Volatile::new(empty_char()))),
        }
    }

    fn empty_char() -> ScreenChar {
        ScreenChar {
            ascii_character: b' ',
            colour_code: ColourCode::new(Colour::Blue, Colour::Magenta, false),
            // Must be the same colour, because Writer clears the screen on initialisation
        }
    }

    #[test]
    fn test_write_byte() {
        let mut writer = construct_writer();
        writer.write_byte(b'X');
        writer.write_byte(b'Y');

        for (i, row) in writer.buffer.chars.iter().enumerate() {
            for (j, screen_char) in row.iter().enumerate() {
                let screen_char = screen_char.read();
                if i == 0 && j == 0 {
                    assert_eq!(screen_char.ascii_character, b'X');
                    assert_eq!(screen_char.colour_code, writer.colour_code);
                } else if i == 0 && j == 1 {
                    assert_eq!(screen_char.ascii_character, b'Y');
                    assert_eq!(screen_char.colour_code, writer.colour_code);
                } else {
                    assert_eq!(screen_char, empty_char());
                }
            }
        }
    }
}
