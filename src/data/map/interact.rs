use std::rc::Rc;

use crate::utils::*;
use crate::data::{Data, State};

pub type InteractAction = dyn Fn(&mut Data) -> ();

pub struct Interact {
    pub name: Option<String>,
    pub description: String,
    pub action: Rc<InteractAction>,
}
impl Default for Interact {
    fn default() -> Self {
        Self {
            name: None,
            description: String::from("Default action"),
            action: Rc::new(|_d| ())
        }
    }
}
impl Interact {
    pub fn draw(&self, state: &mut State) {
        let (screen_w, screen_h) = (getmaxx(state.win) as usize, getmaxy(state.win) as usize);

        let x = screen_w / 16;
        let state_y = (screen_h - (crate::MAP_ROWS*crate::SCALE.1))/3 + crate::SCALE.1*12;
        let y = (state_y + screen_h) / 2;
        let w = screen_w * 1/16;
        println!("{}{}", Goto(state_y+1, x*13), "~".repeat(w*2));
        if let Some(name) = &self.name {
            println!("{}{}", Goto(y-1, x*13), pad(name, ' ', w*2))
        }
        println!("{}{}", Goto(y, x*13), pad(&format!("{}{}press {}<enter>{} to{}{}", LightBlack, Italic, Blink, NotBlink, NotItalic, DefaultColor), ' ', w*2));
        println!("{}{}", Goto(y+1, x*13), pad(&self.description, ' ', w*2));
        println!("{}{}", Goto(screen_h - 1, x*13), "~".repeat(w*2));
    }
}