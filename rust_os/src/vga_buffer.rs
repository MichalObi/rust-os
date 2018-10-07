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
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
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
                self.buffer.chars[row][col] =  ScreenChar {
                    ascii_character: byte,
                    color_code: color_code,
                };

                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {}

    pub fn write_string(&mut self, string: &str) {
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
    let mut writter = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Pink, Color::Black),
        buffer: unsafe { &mut *(video_memory_location as *mut Buffer) },
    };

    writter.write_string(text);
}
