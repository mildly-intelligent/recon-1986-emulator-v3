//! Module for the map data and room numbers

mod interact;

use crate::utils::*;
use crate::data::{State, Data};
use interact::{ConstInteract, Interact};

pub struct ConstMap {
    lines: [&'static str; crate::MAP_ROWS],
    interacts: [ConstInteract; 16],
}
impl ConstMap {
    pub const fn new(lines: [&'static str; crate::MAP_ROWS]) -> Self {
        ConstMap {
            lines,
            interacts: [ConstInteract::default(); 16],
        }
    }

    /// Sets an interact with a given index to an interact
    pub const fn set_interact(self, index: u8, name: Option<&'static str>, description: &'static str, action: fn(&mut Data)) -> Self {
        let mut new = self;
        new.interacts[hex_const(index)] = ConstInteract {
            name,
            description,
            action,
        };

        new
    }

    /// Creates a duplicate interact 
    pub const fn dup_interact(self, index_new: u8, index_old: u8) -> Self {
        let mut new = self;
        let old_interact = new.interacts[hex_const(index_old)];

        new.interacts[hex_const(index_new)] = old_interact;
        
        new
    }
}
impl Into<Map> for ConstMap {
    fn into(self) -> Map {
        let interacts: Vec<Interact> = self.interacts.map(ConstInteract::into).to_vec();
        let interacts = interacts.iter().map(|i| -> Interact {
            let mut i = i.clone();
            i.name = i.name.and_then(|a| a.wrap(Bold).ok());
            i
        }).collect();
        Map {
            lines: self.lines.map(str::to_string).to_vec(),
            interacts,
        }
    }
}

// #[derive(Debug)]
/// A single room with interacts
pub struct Map {
    /// Vector of lines in the map
    lines: Vec<String>,
    /// A dictionary containing the indices and actions for the map's interacts
    // interacts: HashMap<u8, Interact>,
    interacts: Vec<Interact>,
}
impl Map {
    /// Creates a new map with the given lines
    pub fn new(lines: [&str; crate::MAP_ROWS]) -> Self {
        Map {
            lines: lines.map(str::to_string).to_vec(),
            interacts: vec![],
        }
    }

    /// Gets the number of rows in the map
    pub fn rows(&self) -> usize { self.lines.len() }
    /// Gets the number of columns in the map
    pub fn cols(&self) -> usize { self.lines.iter().map(|a| a.len()).max().unwrap() }

    /// Gets the character at a given position (y, x)
    pub fn at(&self, pos: (usize, usize)) -> Option<u8> {
        Some(*self.lines.get(pos.0 as usize)?.as_bytes().get(pos.1 as usize)?)
    }

    /// Draws the map to the screen
    pub fn draw(&self, state: &mut State, player: &mut super::player::Player) -> Result<(), ()> {
        let (w,h) = crate::SCALE;
        let x = (getmaxx(state.win) as usize - (self.cols()*w))/2;
        let y = (getmaxy(state.win) as usize - (self.rows()*h))/3;

        for (i, line) in self.lines.iter().enumerate() {
            let mut line = line.clone();
            duplicate_char(&mut line, ' ', " ", w);
            duplicate_char(&mut line, '-', "-", w);
            duplicate_char(&mut line, '|', "|", w);
            duplicate_char(&mut line, '+', "+", w);
            duplicate_char(&mut line, '#', "#", w);
            duplicate_char(&mut line, 'X', "X", w);

            if let Some(c) = player.interact {
                duplicate_char(&mut line, c as char, "*", w);
                self.get_interact(c).unwrap().draw(state)?;
            }

            for n in '0'..='9' {
                duplicate_char(&mut line, n, "?", w);
            }
            for n in 'A'..='F' {
                duplicate_char(&mut line, n, "?", w);
            }

            // Gang what the fuck is this line 😔✌️
            line = line.replace("*".repeat(w.into()).as_str(), &format!("{}{}{}", Blink, "*".repeat(w.into()), NotBlink).to_string());

            for j in 0..h {
                print!("{}{line}", Goto(h*i+y+j, x));
            }
        }

        Ok(())
    }

    /// Sets an interact with a given index to an interact
    pub fn set_interact(self, index: u8, name: Option<&str>, description: &str, action: fn(&mut Data)) -> Result<Self, ()> {
        let mut new = self;
        new.interacts.insert(
            hex(index)?,
            Interact {
                name: name.and_then(|a| a.wrap(Bold).ok()),
                description: String::from(description),
                action: action,
            }
        );

        Ok(new)
    }

    /// Creates a duplicate interact 
    pub fn dup_interact(self, index_new: u8, index_old: u8) -> Result<Self, ()> {
        let mut new = self;
        let old_interact = new.get_interact(index_old)?;

        new.interacts.insert(
            hex(index_new)?,
            old_interact.clone(),
        );
        
        Ok(new)
    }

    /// Get the interact at an index, returns Err if there is no interact with the index
    pub fn get_interact(&self, index: u8) -> Result<&Interact, ()> {
        self.interacts.get(hex(index)?).ok_or(())
    }
}

#[derive(Clone, Copy)]
/// Enum of the different rooms
pub enum Rooms {
    OpenOcean,
    AirLock,
    Crossways,
    Hall1,
    Hall2,
    Lab1,
    Lab2,
    Lab3,
    Lab4,
    Observatory,
}

use Rooms::*;
/// A specific one way door between rooms
pub struct Entrance(pub Rooms, pub Rooms);
impl Into<Option<(usize, usize)>> for Entrance {
    fn into(self) -> Option<(usize, usize)> {
        // This match is where you're entering and the second is where you came from
        match self.1 {
            OpenOcean => match self.0 {
                AirLock => Some((4,15)),
                _ => None,
            },
            AirLock => match self.0 {
                OpenOcean => Some((10,15)),
                Crossways => Some((1,15)),
                _ => None,
            },
            Crossways => match self.0 {
                AirLock => Some((10,15)),
                Hall1 => Some((3,1)),
                Hall2 => Some((3,28)),
                _ => None,
            },
            Hall1 => todo!(),
            Hall2 => todo!(),
            _ => todo!(),
        }
    }
}
impl std::ops::Index<Rooms> for [Map] {
    type Output = Map;

    fn index(&self, index: Rooms) -> &Self::Output {
        &self[Into::<usize>::into(index as usize)]
    }
}