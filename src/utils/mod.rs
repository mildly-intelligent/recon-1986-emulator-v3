//! Module for miscellaneous utility functions and structs

pub mod overlay;
mod ansi_escape;

use regex::Regex;
use std::sync::LazyLock;

pub use ansi_escape::{Color::*, Style::*, Other::*, AnsiCode};
pub use overlay::Overlay;
pub use ncurses::{getmaxx, getmaxy};

/// Duplicates one character within a string. Example:
/// ```
/// let s = String::from("Hai");
/// duplicate_char(s, 'i', "i", 3);
/// assert_eq!(s, String::from("haiii"));
/// ```
pub fn duplicate_char(s: &mut String, c1: char, c2: &str, n: usize) {
    *s = s.replace(c1, c2.repeat(n).as_str());
}

/// Regex for matching ANSI escape sequences
pub const ANSI_REGEX: LazyLock<Regex> = 
    LazyLock::new(|| Regex::new(r"\x1B\[(?:\d{1,4})?(?:;\d{1,4})*[A-HJm]").unwrap());

#[allow(unused)]
#[doc(alias = "sleep")]
/// Sleep for `millis` milliseconds
pub fn sleep_millis(millis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(millis))
}
#[allow(unused)]
/// Sleep for `micros` microseconds
pub fn sleep_micros(micros: u64) {
    std::thread::sleep(std::time::Duration::from_micros(micros))
}

/// Returns a [usize] from a hexadecimal digit mapping 0-F to 0-15. Returns [Err] if the byte is not a hexadecimal digit.
/// Examples
/// ```
/// assert_eq!(Ok(0xC), hex(b'C'))
/// assert_eq!(Ok(0x3), hex(b'3'))
/// ```
pub fn hex(n: u8) -> Result<usize, ()> {
    {
        if (b'0'..=b'9').contains(&n) {
            Ok(n - b'0')
        } else if (b'A'..=b'F').contains(&n) {
            Ok(n - b'A' + 10)
        } else {
            Err(())
        }
    }.and_then(|a| Ok(a as usize))
}
pub const fn hex_const(n: u8) -> usize {
    if b'0' <= n && n <= b'9' {
        (n - b'0') as usize
    } else if b'A' <= n && n <= b'F' {
        (n - b'A' + 10) as usize
    } else {
        panic!("")
    }
}

/// Trait to implement utility methods on strings and string-like types
pub trait StringUtils:
std::fmt::Display + AsRef<str> + ToString
{
    /// Wraps the string in some [AnsiCode] escape.
    /// This is equivalent to putting some escape then a string, then the inverse of the string.
    /// Examples:
    /// ```
    /// assert_eq!("hello".wrap(Bold).unwrap(), "\x1b[1mhello\x1b[22m")
    /// assert_eq!("test".wrap(Italic).unwrap(), format!("{}test{}", Italic, NotItalic))
    /// ```
    /// See also: [AnsiCode::not]
    fn wrap(&self, code: impl AnsiCode) -> Result<String, ()> {
        Ok(format!("{}{self}{}", self, code.not()?))
    }

    /// Pad a string in the center of a string of `c` with length `n`.
    /// Examples
    /// ```
    /// let s1 = String::from("hi");
    /// let s2 = String::from("test");
    /// assert_eq!(pad(s1, ' ', 6), String::from("  hi  "));
    /// assert_eq!(pad(s1, '~', 6), String::from("~~hi~~"));
    /// assert_eq!(pad(s2, '~', 8), String::from("~~test~~"));
    /// assert_eq!(pad("hello", 'h', 9), "hhhellohh");
    /// ```
    fn pad(&self, c: char, n: usize) -> String {
        let len = ANSI_REGEX.replace_all(&self.to_string(), "").len();
        let pad = c.to_string().repeat((n - len)/2);
        format!("{}{self}{}", pad, pad)
    }
}

impl StringUtils for String {}
impl StringUtils for &str {}