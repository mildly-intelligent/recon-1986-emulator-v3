mod interact;

use crate::utils::*;
use crate::State;
use interact::Interact;

use std::collections::HashMap;

// #[derive(Debug)]
pub struct Map{
    lines: Vec<String>,
    interacts: HashMap<u8, Interact>,
}

impl Map {
    pub fn new(lines: [&str; crate::MAP_ROWS]) -> Self {
        Map {
            lines: lines.map(|a| String::from(a)).to_vec(),
            interacts: HashMap::new(),
        }
    }

    pub fn rows(&self) -> usize { self.lines.len() }
    pub fn cols(&self) -> usize { self.lines.iter().map(|a| a.len()).max().unwrap() }

    pub fn at(&self, pos: (u16, u16)) -> Option<u8> {
        Some(*self.lines.get(pos.0 as usize)?.as_bytes().get(pos.1 as usize)?)
    }

    pub fn draw(&self, state: &mut State, player: &mut crate::player::Player) {
        let (w,h) = crate::SCALE;
        let x = (getmaxx(state.win) as u16 - (self.cols() as u16*w))/2;
        let y = (getmaxy(state.win) as u16 - (self.rows() as u16*h))/3;

        for (i, line) in self.lines.iter().enumerate() {
            let mut line = line.clone();
            repeat_char(&mut line, ' ', " ", w);
            repeat_char(&mut line, '-', "-", w);
            repeat_char(&mut line, '|', "|", w);
            repeat_char(&mut line, '+', "+", w);
            repeat_char(&mut line, '#', "#", w);

            if let Some(c) = player.interact {
                repeat_char(&mut line, c as char, "*", w);
                self.get_interact(c).unwrap().draw(state);
            }

            for n in '0'..='9' {
                repeat_char(&mut line, n, "?", w);
            }
            for n in 'A'..='F' {
                repeat_char(&mut line, n, "?", w);
            }

            // Gang what the fuck is this line 😔✌️
            line = line.replace("*".repeat(w.into()).as_str(), &format!("{}{}{}", Blink, "*".repeat(w.into()), NotBlink).to_string());

            for j in 0..h {
                print!("{}{line}", Goto(h*i as u16+y+j, x));
            }
        }
    }

    pub fn set_interact<F>(self, index: u8, name: Option<&str>, description: &str, action: F) -> Self
    where F: Fn(&mut crate::Data) -> () + 'static
    {
        let mut new = self;
        new.interacts.insert(
            index,
            Interact {
                name: name.and_then(|a| Some(format!("{}{}{}", Bold, a, NotBold))),
                description: String::from(description),
                action: Box::new(action),
            }
        );

        new
    }

    pub fn get_interact(&self, index: u8) -> Option<&Interact> {
        self.interacts.get(&index)
    }
}