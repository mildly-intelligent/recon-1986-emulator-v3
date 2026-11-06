//! Main game loop code

#[cfg(not(debug_assertions))]
pub mod startup;

use crate::utils::*;
use crate::data::{Data, State, map::{Map, Rooms}, player::Player};
use crate::inputs::Context;

#[cfg(not(debug_assertions))]
use crate::inputs::handle_input;
#[cfg(not(debug_assertions))]
use ncurses::getch;

/// Number of rooms in the game
const ROOM_COUNT: usize = 5;

/// Initializes global state and variables
pub fn init(win: *mut i8) -> Result<(State, Player, [Map; ROOM_COUNT], (usize, usize)), ()> {
    let state = State {
        win,
        current_map: Rooms::Crossways,
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
        ]).set_interact(b'0', Some("Outer Airlock"), "Enter Airlock", |d| d.map(Rooms::AirLock)),
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
        ]).set_interact(b'0', Some("Outer Airlock"), "Exit Airlock", |d| d.map(Rooms::OpenOcean))
          .set_interact(b'1', Some("Inner Airlock"), "Enter Vessel", |d| d.map(Rooms::Crossways)),
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
        ]).set_interact(b'0', Some("Inner Airlock"), "Leave Vessel Airlock", |d| d.map(Rooms::AirLock))
          .set_interact(b'1', Some("Doorway"), "Go to Hallway 1", |d| d.map(Rooms::Hall1))
          .set_interact(b'2', Some("Doorway"), "Go to Hallway 2", |d| d.map(Rooms::Hall2))
          .set_interact(b'3', Some("Map"), "Examine", |d| Overlay::print_new(d, "It's a map of the vessel.\n-----\nPressure suits : You are here\nAirlock : VVVV\nLabs 1-4 : <---\nObservatory : <---\nQuarters : --->\nCafeteria : --->\nLounge : --->"))
          .set_interact(b'4', Some("Pressure suit"), "Examine", |d| Overlay::print_new(d, "This is where the crew got the suits to go outside.\nWell, most of the crew."))
          .dup_interact(b'5', b'4', None)?
          .dup_interact(b'6', b'4', None)?
          .dup_interact(b'7', b'4', None)?,
        Map::new([
            "    |444444|   |111111|         ",
            "    |      |   |      |         ",
            "    |      |   |      |         ",
            "-----      -----      -------   ",
            "5                           ----",
            "5                              0",
            "5                              0",
            "5                           ----",
            "-----      -----      -------   ",
            "    |      |   |      |         ",
            "    |      |   |      |         ",
            "    |333333|   |222222|         ",
        ]).set_interact(b'0', Some("Door"), "Enter Main Antechamber", |d| d.map(Rooms::Crossways))
          .set_interact(b'1', Some("Door"), "Enter Lab 1", |d| d.map(Rooms::Lab1))
          .set_interact(b'2', Some("Door"), "Enter Lab 2", |d| d.map(Rooms::Lab2))
          .set_interact(b'3', Some("Door"), "Enter Lab 3", |d| d.map(Rooms::Lab3))
          .set_interact(b'4', Some("Door"), "Enter Lab 4", |d| d.map(Rooms::Lab4))
          .set_interact(b'5', Some("Door"), "Enter Observatory", |d| d.map(Rooms::Observatory)),
        Map::new([
            "                                ",
            "                                ",
            "                                ",
            "                                ",
            "                                ",
            "                                ",
            "                                ",
            "                                ",
            "                                ",
            "                                ",
            "                                ",
            "                                ",
        ]),
    ];

    Ok((state, player, maps, (screen_w, screen_h)))
}

#[cfg(not(debug_assertions))]
/// Sequence of visual events to do before starting the game. Does, in order:
/// 1. Runs the [startup sequence](startup::startup_sequence()),
/// 2. Displaying the title car
/// 3. Clears the screen and sets starting context
/// 
/// 1 and 2 are disabled in debug mode.
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

/// Sequence of visual events to do before starting the game. Does, in order:
/// 1. Runs the [startup sequence](startup::startup_sequence()),
/// 2. Displaying the title car
/// 3. Clears the screen and sets starting context
/// 
/// 1 and 2 are disabled in debug mode.
#[cfg(debug_assertions)]
pub fn start(data: &mut Data, (_w, _h): (usize, usize)) {
    println!("{}{}\x1b[?25l", Clear, Home);
    data.draw();
    data.0.ctx = Context::Map;
}