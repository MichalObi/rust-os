use volatile::Volatile;
use core::fmt; // format core functionality
use spin::Mutex; // Mutex from extern crate (no std here !)

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[allow(dead_code)] // disabled unused code warnings for Color variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // we can derive some common traits for our need
#[repr(u8)] // set type layout of enum
pub enum Color {
    Black=0,
    Blue=1,
    Green=2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8); // u8 is value from 0 - 255 (2^8)

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((foreground as u8) | (background as u8) << 4) // << left-shift operator (?)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // C type layout quarantee fields order in enum
struct ScreenChar {
    ascii_character: u8, // ASCII code point - actuale letter
    color_code: ColorCode, // contain foreground and background
}

const BUFFER_HEIGHT: usize = 25; // VGA buffer array rows size
const BUFFER_WIDTH: usize = 80; // VGA buffer columns size

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT], // use generic Volatile
}

pub struct Writer {
    column_position: usize, // current position in the last row
    color_code: ColorCode,
    buffer: &'static mut Buffer, // 'static tell compiler that Buffer will be valid fot the whole program runtime
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(), // if user provide new line (\n) - go to new line fn
            byte => {
                if self.column_position >= BUFFER_WIDTH { // if is last line
                    self.new_line(); // go to new line fn
                }

                let row = BUFFER_HEIGHT - 1; // current row
                let col = self.column_position; // current column
                let color_code = self.color_code; // current color

                // write to the buffer at the current position
                self.buffer.chars[row][col].write(ScreenChar { // usage of write prevent to agresive compiler optimization
                    ascii_character: byte,
                    color_code: color_code,
                });

                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1); // row off the screen
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) { // override with space character
        // empty character
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_str(&mut self, string: &str) {
        for byte in string.bytes() {
            let empty_char = 0xfe;
            match byte {
                0x20...0x7e | b'\n' => self.write_byte(byte), // ASCII | new line
                _ => self.write_byte(empty_char), //not ASCII
            }
        }
    }
}

// tmp fn to test Writer struct
pub fn print_test_text(text: &str) {
    // address of vga memory location
    let video_memory_location = 0xb8000;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Pink, Color::Black),
        buffer: unsafe { &mut *(video_memory_location as *mut Buffer) },
    };

    writer.write_str(text);
}


// This macro will help print different types
impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write_str(string);
        Ok(())
    }
}


macro_rules! print {
    // use vga_buffer public method to print
    ($($arg:tt)*) => ($crate::vga_buffer::print(format_args!($($arg)*)));
}

macro_rules! println {
    // empty line, if no param passed like println!()
    () => (print!("\n"));
    // print with one arg like println!("TEST")
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
     // print with multiple param like println!("Numbers: {}{}", 1, 2)
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap(); // unwrap make panic if printic not happened well
}

pub fn print_with_macro_test() {
    use core::fmt::Write;
    let video_memory_location = 0xb8000;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Pink, Color::Black),
        buffer: unsafe { &mut *(video_memory_location as *mut Buffer) },
    };
    write!(writer, "The numbers for {} are {}", "test", 2.0/3.0).unwrap();
}

/* Unit tests */

#[cfg(test)]
mod test {
    use super::*; // import vga_buffer functions to test

    // First create main items based on Struct - Wirter, Buffer, ScreenChar

    fn construct_writer() -> Writer {
        /* Box will help to store date on heap - pointer will remain
           on stack - buffer will need to be &'static mut ike in struct definition */

        use std::boxed::Box;

        let buffer = construct_buffer();

        /*
            Box new will allocate memory on heap and place buffer there.
            Box leak will return mutable reference that we need
        */

        Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::Yellow, Color::White),
            buffer: Box::leak(Box::new(buffer))
        }
    }

    fn construct_buffer() -> Buffer {
        Buffer {
            chars: [[Volatile::new(empty_char()); BUFFER_WIDTH]; BUFFER_HEIGHT]
        }
    }

    fn empty_char() -> ScreenChar {
        ascii_character: b' ',
        color_code: ColorCode::new(Color::Yellow, Color::White)
    }
}
