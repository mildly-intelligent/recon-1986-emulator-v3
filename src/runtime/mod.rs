#[cfg(not(debug_assertions))]
pub mod startup;

use ncurses::getch;

use crate::utils::*;

use crate::data::map::Maps;
use crate::data::{Data, State, map::Map, player::Player};
use crate::inputs::{handle_input, Context};

const ROOM_COUNT: usize = 3;
pub fn init(win: *mut i8) -> Result<(State, Player, [Map; ROOM_COUNT], (usize, usize)), ()> {
    let state = State {
        win,
        current_map: Maps::Crossways,
        ctx: Context::Title,
        current_objective: String::from("Explore"),
        inventory: vec![
            String::from("Swiss army knife"),
            String::from("Cheese"),
            String::from("Flashlight"),
        ]
    };

    let screen_w = getmaxx(state.win) as usize;
    let screen_h = getmaxy(state.win) as usize;

    let player = Player {
        pos: (6,17),
        interact: None
    };

    let maps: [Map; ROOM_COUNT] = [
        Map::new([ 
            "----------                 -----",
            "|  XXX   |              ----   |",
            "|   XX   |            ---      |",
            "|   X    -----0000-----        |",
            "|                              |",
            "|                              |",
            "|        X               X     |",
            "|      XXXXX             XXX   |",
            "|     XXX  X             XX    |",
            "|       XX                     |",
            "|        X                     |",
            "--------------------------------",
        ]).set_interact(b'0', Some("Outer Airlock"), "Enter Airlock", |d| d.map(Maps::AirLock)),
        Map::new([
            "           |11111111|           ",
            "           |        |           ",
            "           |        |           ",
            "           ----++----           ",
            "               ##               ",
            "               ##               ",
            "               ##               ",
            "               ##               ",
            "           ----++----           ",
            "           |        |           ",
            "           |        |           ",
            "           ---0000---           ",
        ]).set_interact(b'0', Some("Outer Airlock"), "Exit Airlock", |d| d.map(Maps::OpenOcean))
          .set_interact(b'1', Some("Inner Airlock"), "Enter Vessel", |d| d.map(Maps::Crossways)),
        Map::new([
            "       ------------------       ",
            "       |      3333      --------",
            "--------                       2",
            "1                              2",
            "1                              2",
            "--------                --------",
            "       |6              7|       ",
            "       |6              7|       ",
            "       |                |       ",
            "       |4              5|       ",
            "       |4              5|       ",
            "       -----00000000-----       ",
        ]).set_interact(b'0', Some("Inner Airlock"), "Leave Vessel Airlock", |d| d.map(Maps::OpenOcean))
          .set_interact(b'1', Some("Inner Airlock"), "Leave Vessel Airlock", |d| d.map(Maps::OpenOcean))
          .set_interact(b'2', Some("Inner Airlock"), "Leave Vessel Airlock", |d| d.map(Maps::OpenOcean))
          .set_interact(b'3', Some("Map"), "Examine", |d| Overlay::print_new_ctx(d, "It's a map of the vessel.\n-----\nPressure suits : You are here\nAirlock : VVV\nTODO : <---\nTODO : --->"))
          .set_interact(b'4', Some("Pressure suit"), "Examine", |d| Overlay::print_new_ctx(d, "This is where the crew got the suits to go outside.\nWell, most of the crew."))
          .dup_interact(b'5', b'4', None)?
          .dup_interact(b'6', b'4', None)?
          .dup_interact(b'7', b'4', None)?,
    ];

    Ok((state, player, maps, (screen_w, screen_h)))
}

#[cfg(not(debug_assertions))]
pub fn start(data: &mut Data, (w, h): (usize, usize)) {
    startup::startup_sequence();
    
    println!("{}{}\x1b[?25l", Clear, Home);
    let title_card = overlay::Overlay::from(data.0, &format!("  __ __       _____    _____     _____      __   __   
 /_/\\__/\\   /\\_____\\  /\\ __/\\   ) ___ (    /_/\\ /\\_\\  
 ) ) ) ) ) ( (_____/  ) )__\\/  / /\\_/\\ \\   ) ) \\ ( (  
/_/ /_/_/   \\ \\__\\   / / /    / /_/ (_\\ \\ /_/   \\ \\_\\ 
\\ \\ \\ \\ \\   / /__/_  \\ \\ \\_   \\ \\ )_/ / / \\ \\ \\   / / 
 )_) ) \\ \\ ( (_____\\  ) )__/\\  \\ \\/_\\/ /   )_) \\ (_(  
 \\_\\/ \\_\\/  \\/_____/  \\/___\\/   )_____(    \\_\\/ \\/_/  \n\n{}\n{}",
    pad(&String::from("Erimita Interactive LLC."), ' ', 54),
    pad(&String::from("c. 1986"), ' ', 54)));
    let mut pos3000: (usize,usize) = (1,1);
    while !handle_input(getch(), data) {
        for i in 0..3000 {
            pos3000.0 = (pos3000.0 * 2791 + pos3000.1) % (h+1);
            pos3000.1 = (pos3000.1 * 7129 + pos3000.0) % ((w+1)/4);
            print!("{}{}3000{}", Goto(pos3000.0, pos3000.1*4), C256((i%5) as u8+234), DefaultColor);
        }
        title_card.draw();
        println!("{}{}{}Insert 5¢ to begin{}", Down(3), Column((w-18)/2), Blink, NotBlink);
    }

    data.draw();
    data.0.ctx = Context::Map;
}

#[cfg(debug_assertions)]
pub fn start(data: &mut Data, (_w, _h): (usize, usize)) {
    println!("{}{}\x1b[?25l", Clear, Home);
    data.draw();
    data.0.ctx = Context::Map;
}