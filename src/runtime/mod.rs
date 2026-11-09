//! Main game loop code

#[cfg(not(debug_assertions))]
pub mod startup;

use crate::utils::*;
use crate::data::{Data, State, map::{Map, ConstMap, Rooms}, player::Player};
use crate::inputs::Context;

#[cfg(not(debug_assertions))]
use crate::inputs::handle_input;
#[cfg(not(debug_assertions))]
use ncurses::getch;

/// Number of rooms in the game
const ROOM_COUNT: usize = 11;

const OPEN_OCEAN: ConstMap = ConstMap::new([ 
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
]).set_interact(b'0', Some("Outer Airlock"), "Enter Airlock", |d| d.map(Rooms::AirLock));
const AIR_LOCK: ConstMap = ConstMap::new([
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
  .set_interact(b'1', Some("Inner Airlock"), "Enter Vessel", |d| d.map(Rooms::Crossways));
const CROSSWAYS: ConstMap = ConstMap::new([
    "       ------------------       ",
    "       |      3333      --------",
    "--------                X      2",
    "1                       X      2",
    "1                       X      2",
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
  .dup_interact(b'5', b'4')
  .dup_interact(b'6', b'4')
  .dup_interact(b'7', b'4');
const HALLWAY_1: ConstMap = ConstMap::new([
    "    |333333|   |222222|         ",
    "    |      |   |      |         ",
    "    |      |   |      |         ",
    "-----XXXXXX-----XXXXXX-------   ",
    "5   X                       ----",
    "5   X                          0",
    "5   X                          0",
    "5   X                       ----",
    "-----      -----XXXXXX-------   ",
    "    |      |   |      |         ",
    "    |      |   |      |         ",
    "    |444444|   |111111|         ",
]).set_interact(b'0', Some("Door"), "Enter Main Antechamber", |d| d.map(Rooms::Crossways))
  .set_interact(b'1', Some("Door"), "Enter Lab 1", |d| d.map(Rooms::Lab1))
  .set_interact(b'2', Some("Door"), "Enter Lab 2", |d| d.map(Rooms::Lab2))
  .set_interact(b'3', Some("Door"), "Enter Lab 3", |d| d.map(Rooms::Lab3))
  .set_interact(b'4', Some("Door"), "Enter Lab 4", |d| d.map(Rooms::Lab4))
  .set_interact(b'5', Some("Door"), "Enter Observatory", |d| d.map(Rooms::Observatory));
const HALLWAY_2: ConstMap = ConstMap::default();
const LAB_1: ConstMap = ConstMap::default();
const LAB_2: ConstMap = ConstMap::default();
const LAB_3: ConstMap = ConstMap::default();
const OBSERVATORY: ConstMap = ConstMap::default();
const LAB_4: ConstMap = ConstMap::new([
    "             |0000|             ",
    "             |    |             ",
    "     ---------    ---------     ",
    "     |               4    |     ",
    "     |2             4    3|     ",
    "     |2                  3|     ",
    "     |2                  3|     ",
    "     |       111111       |     ",
    "     ----------------------     ",
    "                                ",
    "                                ",
    "                                ",
]).set_interact(b'0', Some("Door"), "Leave lab 4", |d| d.map(Rooms::Hall1))
  .set_interact(b'1', Some("Table"), "Examine", |d| Overlay::print_new(d, &format!("There's a bunch of lab equipment.\nYou can't seem to {} what it's for.", "remember".wrap(LightGreen).unwrap())))
  .dup_interact(b'2', b'1')
  .dup_interact(b'3', b'1')
  .set_interact(b'4', Some("Chemical spill"), "Examine", |d| {
    d.0.y909 = true;
    d.0.current_objective = "Remember".wrap(LightGreen).unwrap();
    Overlay::print_new(d, &format!("There's some spillage of chemicals.\nAnd many shards of broken glass that seem to have belonged to a beaker of some kind.\nOne shard has a label, it says {}.\n{}", "Y-909".wrap(Bold).unwrap().wrap(LightGreen).unwrap(), "What were you doing in here again?".wrap(Italic).unwrap().wrap(LightBlack).unwrap()));
});
const HALLWAY_1_ALT: ConstMap = ConstMap::new([
    "    |000000|                    ",
    "    |      |                    ",
    "    |      |                    ",
    "    |      |                    ",
    "    |      |                    ",
    "    |      |                    ",
    "    |      |                    ",
    "    |      |                    ",
    "    |      |                    ",
    "    |      |                    ",
    "    |      |                    ",
    "    |      |                    ",
]).set_interact(b'0', Some("???"), "Go on", |d| d.map(Rooms::Hall1Alt));

/// List of rooms in the game.
pub const ROOMS: [ConstMap; ROOM_COUNT] = [
    OPEN_OCEAN,
    AIR_LOCK,
    CROSSWAYS,
    HALLWAY_1,
    HALLWAY_2,
    LAB_1,
    LAB_2,
    LAB_3,
    LAB_4,
    OBSERVATORY,
    HALLWAY_1_ALT,
];

/// Initializes global state and variables
pub fn init(win: *mut i8) -> Result<(State, Player, [Map; ROOM_COUNT]), ()> {
    let state = State {
        win,
        current_map: Rooms::OpenOcean,
        ctx: Context::Title,
        current_objective: String::from("Explore"),
        inventory: vec![
            String::from("Swiss army knife"),
            String::from("Cheese"),
            String::from("Flashlight"),
        ],
        y909: false,
    };

    let player = Player {
        pos: (6,17),
        interact: None
    };

    let maps: [Map; ROOM_COUNT] = ROOMS.map(|a| a.into());

    Ok((state, player, maps))
}

#[cfg(not(debug_assertions))]
/// Sequence of visual events to do before starting the game. Does, in order:
/// 1. Runs the [startup sequence](startup::startup_sequence()),
/// 2. Displaying the title car
/// 3. Clears the screen and sets starting context
/// 
/// 1 and 2 are disabled in debug mode.
pub fn start(data: &mut Data) -> Result<(), ()> {
    let (w,h) = (getmaxx(data.0.win) as usize, getmaxy(data.0.win) as usize);

    startup::startup_sequence()?;
    
    println!("{}{}\x1b[?25l", Clear, Home);
    let mut pos3000: (usize,usize) = (1,1);
    while !handle_input(getch(), data) {
        for i in 0..3000 {
            pos3000.0 = (pos3000.0 * 2791 + pos3000.1) % (h+1);
            pos3000.1 = (pos3000.1 * 7129 + pos3000.0) % ((w+1)/4);
            print!("{}{}", Goto(pos3000.0, pos3000.1*4), "3000".wrap(C256((i%5) as u8+234))?);
        }
        overlay::Overlay::print_new_no_ctx(data, &format!("  __ __       _____    _____     _____      __   __   
 /_/\\__/\\   /\\_____\\  /\\ __/\\   ) ___ (    /_/\\ /\\_\\  
 ) ) ) ) ) ( (_____/  ) )__\\/  / /\\_/\\ \\   ) ) \\ ( (  
/_/ /_/_/   \\ \\__\\   / / /    / /_/ (_\\ \\ /_/   \\ \\_\\ 
\\ \\ \\ \\ \\   / /__/_  \\ \\ \\_   \\ \\ )_/ / / \\ \\ \\   / / 
 )_) ) \\ \\ ( (_____\\  ) )__/\\  \\ \\/_\\/ /   )_) \\ (_(  
 \\_\\/ \\_\\/  \\/_____/  \\/___\\/   )_____(    \\_\\/ \\/_/  \n\n{}\n{}",
            "Erimita Interactive LLC.".pad(' ', 54),
            "c. 1986".pad(' ', 54)
        ));
        println!("{}{}{}", Down(3), Column((w-18)/2), "Insert 5¢ to begin".wrap(Blink)?);
    }

    data.draw()?;
    data.0.ctx = Context::Map;

    Ok(())
}

/// Sequence of visual events to do before starting the game. Does, in order:
/// 1. Runs the [startup sequence](startup::startup_sequence()),
/// 2. Displaying the title car
/// 3. Clears the screen and sets starting context
/// 
/// 1 and 2 are disabled in debug builds.
#[cfg(debug_assertions)]
pub fn start(data: &mut Data) -> Result<(), ()> {
    println!("{}{}\x1b[?25l", Clear, Home);
    data.draw()?;
    data.0.ctx = Context::Map;

    Ok(())
}