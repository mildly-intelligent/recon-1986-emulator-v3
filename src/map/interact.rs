use crate::utils::*;
use crate::{State, map::Map, player::Player};

pub type InteractAction = dyn Fn(&mut State, &mut Player, &Map) -> ();

pub struct Interact {
    pub name: Option<String>,
    pub description: String,
    pub action: Box<InteractAction>,
}
impl Default for Interact {
    fn default() -> Self {
        Self {
            name: None,
            description: String::from("Default action"),
            action: Box::new(|_state,_p,_m| ())
        }
    }
}
impl Interact {
    pub fn draw(&self, state: &mut State) {
        let (screen_w, screen_h) = (getmaxx(state.win) as u16, getmaxy(state.win) as u16);

        let x = screen_w / 16;
        let state_y = (screen_h - (crate::MAP_ROWS as u16*crate::SCALE.1))/3 + crate::SCALE.1*12;
        let y = (state_y + screen_h) / 2;
        let w = screen_w * 1/8;
        println!("{}{}", Goto(state_y+1, x*7), "~".repeat(w as usize));
        if let Some(name) = &self.name {
            println!("{}{}", Goto(y-1, x*7), pad(name, ' ', w as usize))
        }
        println!("{}{}", Goto(y, x*7), pad(&format!("{}{}press {}<enter>{} to{}{}", LightBlack, Italic, Blink, NotBlink, NotItalic, DefaultColor), ' ', w as usize));
        println!("{}{}", Goto(y+1, x*7), pad(&self.description, ' ', w as usize));
        println!("{}{}", Goto(screen_h as u16 - 1, x*7), "~".repeat(w as usize));
    }
}