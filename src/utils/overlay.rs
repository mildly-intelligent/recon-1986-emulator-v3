use crate::inputs;
use crate::utils::*;
use crate::data::{State, Data};

use ncurses::getch;
use regex::Regex;

pub struct Overlay {
    y: usize,
    x: usize,
    h: usize,
    w: usize,
    text: Option<String>,
}
impl Overlay {
    pub fn new(w: *mut i8, width: usize, height: usize) -> Self {
        let screen_w = getmaxx(w) as usize;
        let screen_h = getmaxy(w) as usize;

        Self {
            x: (screen_w - width) / 2,
            y: (screen_h - height) / 2,
            w: width,
            h: height,
            text: None,
        }
    }

    pub fn from(state: &State, text: &str) -> Self {
        let ansi_regex = Regex::new(r"\x1B\[(?:\d{1,4})?(?:;\d{1,4})*[A-HJm]").unwrap();
        let screen_w = getmaxx(state.win) as usize;
        let screen_h = getmaxy(state.win) as usize;

        let split: Vec<&str> = text.split('\n').collect();
        let width = split.iter()
            .map(|a| ansi_regex.replace_all(a, "").chars().count())
            .max()
            .unwrap();
        let height = split.len();

        Self {
            x: (screen_w - width) / 2,
            y: (screen_h - height) / 2,
            w: width,
            h: height,
            text: Some(String::from(text)),
        }
    }

    fn draw_border(&self) {
        let center = format!("{}     | {} |     {}", Column(self.x-7), " ".repeat(self.w), Down(1));
        println!("{}     {}     ", Goto(self.y-2, self.x-7), " ".repeat(self.w+4));
        println!("{}     +{}+     ", Column(self.x-7), "-".repeat(self.w+2));
        println!("{}{}", center.repeat(self.h), Up(1));
        println!("{}     +{}+     ", Column(self.x-7), "-".repeat(self.w+2));
        println!("{}     {}     ", Column(self.x-7), " ".repeat(self.w+4));
    }

    pub fn print_new(data: &mut Data, text: &str) {
        let split: Vec<&str> = text.split('\n').collect();
        let width = split.iter()
            .map(|a| ANSI_REGEX.replace_all(a, "").chars().count())
            .max()
            .unwrap();
        let height = split.len();

        let overlay = Self::new(data.0.win, width, height);

        overlay.print(text, Some(data));
    }

    pub fn print_new_ctx(data: &mut Data, text: &str) {
        data.0.ctx = inputs::Context::Overlay;
        Self::print_new(data, text);
    }

    pub fn print(&self, text: &str, data: Option<&mut Data>) {
        self.draw_border();

        let lines = text.split('\n');
        for (i, line) in lines.enumerate() {
            println!("{}{}", Goto(self.y+i, self.x), line)
        }
        if data.is_some() {
            let data = data.unwrap();
            while !inputs::handle_input(getch(), data) {
                self.draw_border();
    
                let lines = text.split('\n');
                for (i, line) in lines.enumerate() {
                    println!("{}{}", Goto(self.y+i, self.x), line)
                }
            }
        }
    }

    pub fn draw(&self) { self.print(self.text.as_ref().unwrap().as_str(), None) }
}