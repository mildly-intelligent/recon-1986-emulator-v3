//! Module for handling interact objects on the map

use crate::utils::*;
use crate::data::{Data, State};

#[derive(Clone, Copy)]
pub struct ConstInteract {
    /// The name to display in the status bar
    pub name: Option<&'static str>,
    /// Description of the action to display
    pub description: &'static str,
    /// Function to run when clicking on the action
    pub action: fn(&mut Data),
}
impl ConstInteract {
    pub const fn default() -> Self {
        Self {
            name: None,
            description: "Examine",
            action: |_a| ()
        }
    }
}
impl Into<Interact> for ConstInteract {
    fn into(self) -> Interact {
        Interact {
            name: self.name.map(str::to_string),
            description: self.description.to_string(),
            action: self.action,
        }
    }
}

#[derive(Clone)]
/// An interact object in the maps
pub struct Interact {
    /// The name to display in the status bar
    pub name: Option<String>,
    /// Description of the action to display
    pub description: String,
    /// Function to run when clicking on the action
    pub action: fn(&mut Data),
}
impl Default for Interact {
    fn default() -> Self {
        Self {
            name: None,
            description: String::from("Examine"),
            action: |_d| ()
        }
    }
}
impl Interact {
    /// Draws the interact to the status bar
    pub fn draw(&self, state: &mut State) -> Result<(), ()> {
        let (screen_w, screen_h) = (getmaxx(state.win) as usize, getmaxy(state.win) as usize);

        let x = screen_w / 16;
        // I honestly don't remember how this math worked out
        let state_y = (screen_h - (crate::MAP_ROWS*crate::SCALE.1))/3 + crate::SCALE.1*12;
        // Height of the row in the center of the status bar
        let y = (state_y + screen_h) / 2;
        let w = screen_w * 1/16;
        // 
        println!("{}{}", Goto(state_y+1, x*13), "~".repeat(w*2));
        if let Some(name) = &self.name {
            println!("{}{}", Goto(y-1, x*13), name.to_string().pad(' ', w*2))
        }
        println!("{}{}", Goto(y, x*13), format!("press {} to", "<enter>".wrap(Blink)?).wrap(LightBlack)?.wrap(Italic)?.pad(' ', w*2));
        println!("{}{}", Goto(y+1, x*13), self.description.to_string().pad(' ', w*2));
        println!("{}{}", Goto(screen_h - 1, x*13), "~".repeat(w*2));

        Ok(())
    }
}