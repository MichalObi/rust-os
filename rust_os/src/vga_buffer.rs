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
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((foreground as u8) | (background as u8) << 4)
    }
}
