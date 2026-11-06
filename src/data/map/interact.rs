//! Module for handling interact objects on the map

use std::rc::Rc;

use crate::utils::*;
use crate::data::{Data, State};

/// Type alias for actions that interacts can trigger
pub type InteractAction = dyn Fn(&mut Data) -> ();

/// An interact object in the maps
pub struct Interact {
    /// The name to display in the status bar
    pub name: Option<String>,
    /// Description of the action to display
    pub description: String,
    /// Function to run when clicking on the action
    pub action: Rc<InteractAction>,
}
impl Default for Interact {
    fn default() -> Self {
        Self {
            name: None,
            description: String::from("Examine"),
            action: Rc::new(|_d| ())
        }
    }
}
impl Interact {
    /// Draws the interact to the status bar
    pub fn draw(&self, state: &mut State) {
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
            println!("{}{}", Goto(y-1, x*13), pad(name, ' ', w*2))
        }
        println!("{}{}", Goto(y, x*13), pad(&format!("{}{}press {}<enter>{} to{}{}", LightBlack, Italic, Blink, NotBlink, NotItalic, DefaultColor), ' ', w*2));
        println!("{}{}", Goto(y+1, x*13), pad(&self.description, ' ', w*2));
        println!("{}{}", Goto(screen_h - 1, x*13), "~".repeat(w*2));
    }
}