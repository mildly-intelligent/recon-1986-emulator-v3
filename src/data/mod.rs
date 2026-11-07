//! Module for all global game data

pub mod map;
pub mod player;

use crate::utils::*;
use map::{Map, Rooms, Entrance};
use player::Player;

/// Global game state
pub struct State {
    /// ncurses window pointer
    pub win: *mut i8,
    /// The current room the player is in
    pub current_map: Rooms,
    /// See [crates::inputs::Context]
    pub ctx: crate::inputs::Context,
    /// Displays in the status bar
    pub current_objective: String,
    /// List of items in the inventory
    /// Currently not implemented for any reason other than the status bar
    pub inventory: Vec<String>,
}
impl State {
    /// Draws the status bar at the bottom of the screen
    pub fn draw(&mut self) -> Result<(), ()> {
        let (screen_w, screen_h) = (getmaxx(self.win) as usize, getmaxy(self.win) as usize);

        let x = screen_w / 16;
        let state_y = crate::MAP_ROWS*crate::SCALE.1 + 3;
        let y = (state_y + screen_h) / 2;
        let w = screen_w * 1/16;

        println!("{}{}", Goto(state_y+1, x), "~".repeat(w*2));
        println!("{}{}", Goto(y-1, x), "Objective".wrap(Bold)?.pad(' ', w*2));
        println!("{}{}", Goto(y+1, x), self.current_objective.pad(' ', w*2));
        println!("{}{}", Goto(screen_h - 1, x), "~".repeat(w*2));

        println!("{}{}", Goto(state_y+1, x*4), "~".repeat(w*8));
        println!("{}{}", Goto(y-1, x*4), "Inventory".wrap(Bold)?.pad(' ', w*8));
        println!("{}{}", Goto(y+1, x*4), self.inventory.join(" | ").pad(' ', w*8));
        println!("{}{}", Goto(screen_h - 1, x*4), "~".repeat(w*8));

        Ok(())
    }
}
impl AsMut<State> for State {
    fn as_mut(&mut self) -> &mut State {
        self
    }
}

/// Collection of all data (state, player, map) that is sent to most functions
pub struct Data<'a>(pub &'a mut State, pub &'a mut Player, pub &'a Map);
impl<'a> Data<'a> {
    /// Draws all relevant information to the screen
    pub fn draw(&mut self) -> Result<(), ()> {
        let Self(state, player, map) = self;
        print!("{}{}", Clear, Home);
        player.get_interact(map);
        map.draw(state, player)?;
        player.draw(state, map);
        println!();

        Ok(())
    }

    /// Switches map to a different room
    pub fn map(&mut self, map: map::Rooms) {
        let entrance = Entrance(self.0.current_map, map);
        self.0.current_map = map;
        if let Some(pos) = entrance.into() {
            self.1.pos = pos;
        }
    }
}