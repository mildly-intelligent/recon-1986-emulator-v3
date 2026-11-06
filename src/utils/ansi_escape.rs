//! Contains enums for simplifying ANSI escape code use

use std::fmt::Display;

#[allow(dead_code)]
/// Changes the color of text
pub enum Color {
    DefaultColor,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    LightBlack,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White,
    /// Changes color to an 8-bit color
    C256(u8),
}
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::DefaultColor => write!(f, "\x1b[39m"),
            Color::Black => write!(f, "\x1b[30m"),
            Color::Red => write!(f, "\x1b[31m"),
            Color::Green => write!(f, "\x1b[32m"),
            Color::Yellow => write!(f, "\x1b[33m"),
            Color::Blue => write!(f, "\x1b[34m"),
            Color::Magenta => write!(f, "\x1b[35m"),
            Color::Cyan => write!(f, "\x1b[36m"),
            Color::Gray => write!(f, "\x1b[37m"),
            Color::LightBlack => write!(f, "\x1b[90m"),
            Color::LightRed => write!(f, "\x1b[91m"),
            Color::LightGreen => write!(f, "\x1b[92m"),
            Color::LightYellow => write!(f, "\x1b[93m"),
            Color::LightBlue => write!(f, "\x1b[94m"),
            Color::LightMagenta => write!(f, "\x1b[95m"),
            Color::LightCyan => write!(f, "\x1b[96m"),
            Color::White => write!(f, "\x1b[97m"),
            Color::C256(n) => write!(f, "\x1b[38;5;{}m", n),
        }
    }
}

#[allow(dead_code)]
/// Changes the style of the text
pub enum Style {
    /// Disables all styles
    /// To disable one style, use `Not<STYLE>`
    Reset,
    Bold,
    NotBold,
    Italic,
    NotItalic,
    Underline,
    NotUnderline,
    Blink,
    NotBlink,
    Invert,
    NotInvert,
}
impl Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Style::Reset => write!(f, "\x1b[0m"),
            Style::Bold => write!(f, "\x1b[1m"),
            Style::NotBold => write!(f, "\x1b[22m"),
            Style::Italic => write!(f, "\x1b[3m"),
            Style::NotItalic => write!(f, "\x1b[23m"),
            Style::Underline => write!(f, "\x1b[4m"),
            Style::NotUnderline => write!(f, "\x1b[24m"),
            Style::Blink => write!(f, "\x1b[5m"),
            Style::NotBlink => write!(f, "\x1b[25m"),
            Style::Invert => write!(f, "\x1b[7m"),
            Style::NotInvert => write!(f, "\x1b[27m"),
        }
    }
}
impl Style {
    pub fn not(&self) -> Result<Self, ()> {
        match self {
            Style::Bold => Ok(Style::NotBold),
            Style::Italic => Ok(Style::NotItalic),
            Style::Underline => Ok(Style::NotUnderline),
            Style::Blink => Ok(Style::NotBlink),
            Style::Invert => Ok(Style::NotInvert),
            _ => Err(())
        }
    }

    pub fn wrap(&self, s: String) -> Result<String, ()> {
        Ok(format!("{}{s}{}", self, self.not()?))
    }
}

#[allow(dead_code)]
/// Other ANSI escape codes such as cursor movement and screen clearing
pub enum Other {
    /// Goes to position (1,1)
    Home,
    Goto(usize, usize),
    Up(usize),
    Down(usize),
    Right(usize),
    Left(usize),
    Column(usize),
    Next(usize),
    Prev(usize),
    /// Saves the position of the cursor to memory, use [Other::Restore] to move the cursor back
    Save,
    /// Returns the cursor to the position when [Other::Save] was used
    Restore,
    Clear,
}
impl Display for Other {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Other::Home => write!(f, "\x1b[H"),
            Other::Goto(line, column) => write!(f, "\x1b[{};{}H", line, column),
            Other::Up(n) => write!(f, "\x1b[{}A", n),
            Other::Down(n) => write!(f, "\x1b[{}B", n),
            Other::Right(n) => write!(f, "\x1b[{}C", n),
            Other::Left(n) => write!(f, "\x1b[{}D", n),
            Other::Next(n) => write!(f, "\x1b[{}E", n),
            Other::Prev(n) => write!(f, "\x1b[{}F", n),
            Other::Column(n) => write!(f, "\x1b[{}G", n),
            Other::Save => write!(f, "\x1b[s"),
            Other::Restore => write!(f, "\x1b[u"),
            Other::Clear => write!(f, "\x1b[2J\x1b[H"),
        }
    }
}