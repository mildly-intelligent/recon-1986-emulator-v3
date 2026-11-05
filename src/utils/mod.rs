//! Module for miscellaneous utility functions and structs

pub mod overlay;
mod ansi_escape;

use regex::Regex;
use std::sync::LazyLock;

pub use ansi_escape::{Color::*, Style::*, Other::*};
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

/// Pad a string in the center of a string of `c` with length `n`.
/// Examples
/// ```
/// let s1 = String::from("hi")
/// let s2 = String::from("test")
/// assert_eq!(pad(s1, ' ', 6), String::from("  hi  "))
/// assert_eq!(pad(s1, '~', 6), String::from("~~hi~~"))
/// assert_eq!(pad(s2, '~', 8), String::from("~~test~~"))
/// ```
pub fn pad(s: &String, c: char, n: usize) -> String {
    let len = ANSI_REGEX.replace_all(s, "").len();
    let pad = c.to_string().repeat((n - len)/2);
    format!("{}{s}{}", pad, pad)
}

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