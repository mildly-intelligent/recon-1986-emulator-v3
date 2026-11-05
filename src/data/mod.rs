pub mod map;
pub mod player;

use crate::utils::*;
use map::{Map, Maps, Entrance};
use player::Player;

pub struct State {
    pub win: *mut i8,
    pub current_map: Maps,
    pub ctx: crate::inputs::Context,
    pub current_objective: String,
    pub inventory: Vec<String>,
}
impl State {
    pub fn draw(&mut self) {
        let (screen_w, screen_h) = (getmaxx(self.win) as usize, getmaxy(self.win) as usize);

        let x = screen_w / 16;
        let state_y = crate::MAP_ROWS*crate::SCALE.1 + 3;
        let y = (state_y + screen_h) / 2;
        let w = screen_w * 1/16;

        println!("{}{}", Goto(state_y+1, x), "~".repeat(w*2));
        println!("{}{}", Goto(y-1, x), pad(&format!("{}Objective{}", Bold, NotBold), ' ', w*2));
        println!("{}{}", Goto(y+1, x), pad(&self.current_objective, ' ', w*2));
        println!("{}{}", Goto(screen_h - 1, x), "~".repeat(w*2));

        println!("{}{}", Goto(state_y+1, x*4), "~".repeat(w*8));
        println!("{}{}", Goto(y-1, x*4), pad(&format!("{}Inventory{}", Bold, NotBold), ' ', w*8));
        println!("{}{}", Goto(y+1, x*4), pad(&self.inventory.join(" | "), ' ', w*8));
        println!("{}{}", Goto(screen_h - 1, x*4), "~".repeat(w*8));
    }
}
impl AsMut<State> for State {
    fn as_mut(&mut self) -> &mut State {
        self
    }
}

pub struct Data<'a>(pub &'a mut State, pub &'a mut Player, pub &'a Map);
impl<'a> Data<'a> {
    pub fn draw(&mut self) {
        let Self(state, player, map) = self;
        print!("{}{}", Clear, Home);
        player.get_interact(map);
        map.draw(state, player);
        player.draw(state, map);
        println!();
    }

    pub fn map(&mut self, map: map::Maps) {
        let entrance = Entrance(self.0.current_map, map);
        self.0.current_map = map;
        if let Some(pos) = entrance.into() {
            self.1.pos = pos;
        }
    }
}