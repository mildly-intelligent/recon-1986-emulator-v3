mod interact;

use crate::utils::*;
use crate::data::{State, Data};
use interact::Interact;

use std::collections::HashMap;
use std::rc::Rc;

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

    pub fn at(&self, pos: (usize, usize)) -> Option<u8> {
        Some(*self.lines.get(pos.0 as usize)?.as_bytes().get(pos.1 as usize)?)
    }

    pub fn draw(&self, state: &mut State, player: &mut super::player::Player) {
        let (w,h) = crate::SCALE;
        let x = (getmaxx(state.win) as usize - (self.cols()*w))/2;
        let y = (getmaxy(state.win) as usize - (self.rows()*h))/3;

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
                print!("{}{line}", Goto(h*i+y+j, x));
            }
        }
    }

    pub fn set_interact<F>(self, index: u8, name: Option<&str>, description: &str, action: F) -> Self
    where F: Fn(&mut Data) -> () + 'static
    {
        let mut new = self;
        new.interacts.insert(
            index,
            Interact {
                name: name.and_then(|a| Some(format!("{}{}{}", Bold, a, NotBold))),
                description: String::from(description),
                action: Rc::new(action),
            }
        );

        new
    }

    pub fn dup_interact(self, index_new: u8, index_old: u8, new_name: Option<&str>) -> Result<Self, ()> {
        let mut new = self;
        let old_interact = new.get_interact(index_old)?;

        let new_name = new_name.and_then(|a| Some(format!("{}{}{}", Bold, a, NotBold)));
        let description = old_interact.description.clone();
        let action = old_interact.action.clone();
        new.interacts.insert(
            index_new,
            Interact {
                name: new_name.or(old_interact.name.clone()),
                description: String::from(description),
                action,
            }
        );
        
        Ok(new)
    }

    pub fn get_interact(&self, index: u8) -> Result<&Interact, ()> {
        self.interacts.get(&index).ok_or(())
    }
}