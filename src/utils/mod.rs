pub mod ansi_escape;
pub mod overlay;

use regex::Regex;
use std::sync::LazyLock;

pub use ansi_escape::{Color::*, Style::*, Other::*};
pub use overlay::Overlay;
pub use ncurses::{getmaxx, getmaxy};

pub fn repeat_char(s: &mut String, c1: char, c2: &str, n: usize) {
    *s = s.replace(c1, c2.repeat(n).as_str());
}

pub const ANSI_REGEX: LazyLock<Regex> = 
    LazyLock::new(|| Regex::new(r"\x1B\[(?:\d{1,4})?(?:;\d{1,4})*[A-HJm]").unwrap());

pub fn pad(s: &String, c: char, n: usize) -> String {
    let len = ANSI_REGEX.replace_all(s, "").len();
    let pad = c.to_string().repeat((n - len)/2);
    format!("{}{s}{}", pad, pad)
}

#[allow(unused)]
pub fn sleep_secs(secs: u64) {
    std::thread::sleep(std::time::Duration::from_secs(secs))
}
pub fn sleep_millis(millis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(millis))
}
pub fn sleep_micros(micros: u64) {
    std::thread::sleep(std::time::Duration::from_micros(micros))
}
#[allow(non_upper_case_globals)]
pub static sleep: fn(u64) = sleep_millis;