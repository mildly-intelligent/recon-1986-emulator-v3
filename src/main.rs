mod inputs;
mod utils;
mod data;
mod runtime;

use utils::*;
use data::{map::Map, player::Player};

use ncurses::{cbreak, getch, initscr, keypad, noecho};

const SCALE: (u16,u16) = (5,3);
const MAP_ROWS: usize = 12;

fn main() {
    noecho();
    cbreak();

    let mut state = data::State {
        win: initscr(),
        current_map: 0,
        ctx: inputs::Context::Title,
        current_objective: String::from("Explore"),
        inventory: vec![
            String::from("Swiss army knife"),
            String::from("Cheese"),
            String::from("Flashlight"),
        ]
    };
    keypad(state.win, true);

    let screen_w = getmaxx(state.win) as u16;
    let screen_h = getmaxy(state.win) as u16;

    let maps: [Map; 2] = [
        Map::new([
            "  ------        ----333-----    ",
            "  0    ----------          |    ",
            "  0                      1 |    ",
            "  0    ----------          |    ",
            "  ------        -----+------    ",
            "                     #          ",
            "    --------------   #          ",
            "    |            |   #  ------  ",
            "    | 7  5  4  2 |   ###+    |  ",
            "    | 7  5  4  2 |   #  ------  ",
            "    ----------+---   #          ",
            "              ########          ",
        ])
            .set_interact(b'0', Some("Window"), "Look out window?", |d| {d.0.ctx=inputs::Context::Overlay; Overlay::print_new(d, "It's just the vast expanse of the Indian Ocean")})
            .set_interact(b'1', Some("Pile of papers"), "Examine?", |d| {d.0.ctx=inputs::Context::Overlay; Overlay::print_new(d, "Nothing of note, looks like someone dropped them in a hurry")})
            .set_interact(b'2', Some("Bed"), "Examine?", |d| {d.0.ctx=inputs::Context::Overlay; Overlay::print_new(d, "Why you snooping?")})
            .set_interact(b'4', Some("Bed"), "Examine?", |d| {d.0.ctx=inputs::Context::Overlay; Overlay::print_new(d, "Why you snooping?")})
            .set_interact(b'5', Some("Bed"), "Examine?", |d| {d.0.ctx=inputs::Context::Overlay; Overlay::print_new(d, "Why you snooping?")})
            .set_interact(b'7', Some("Bed"), "Examine?", |d| {d.0.ctx=inputs::Context::Overlay; Overlay::print_new(d, &format!("This is honestly concerning, there were four beds that belonged to people you may or may not know.\nFrom a gamer perspective I can get checking out one of them, maybe even two, but {}all four{}?\nThis is honestly disgusting behavior and you should be ashamed of yourself.", Italic, NotItalic))})
            .set_interact(b'3', Some("Door"), "Go to Lab 1?", |d| {d.0.current_map = 1; d.1.pos = (10,16)}),
        Map::new([
            "         ---------------        ",
            "         |             |        ",
            "         |    111      |        ",
            "         |             |        ",
            "         |             |        ",
            "         |---------    |        ",
            "         | 2           |        ",
            "         |         |   |        ",
            "         |    ------   |        ",
            "         |             |        ",
            "         |             |        ",
            "         ------000------        ",
        ])
            .set_interact(b'0', Some("Door"), "Go back main hull?", |d| {d.0.current_map = 0; d.1.pos = (1,19);})
            .set_interact(b'1', Some("Table"), "Examine?", |d| {d.0.ctx=inputs::Context::Overlay; Overlay::print_new(d, "There's just some science equipment, nothing you understand")})
            .set_interact(b'2', Some("Broken glass"), "Examine?", |d| {d.0.ctx=inputs::Context::Overlay; Overlay::print_new(d, &format!("A puddle of mostly clear liquid.\nIn the puddle there's shards you see shards of a glass vial labeled {}Y-909{}\n{}{}What were you looking for again?{}{}", Bold, NotBold, Gray, Italic, NotItalic, DefaultColor))}),
    ];

    let mut player = Player {
        pos: (3,17),
        interact: None
    };
    let map = &maps[state.current_map];

    let mut data = data::Data(&mut state, &mut player, &map);
    

    if !cfg!(debug_assertions) { runtime::startup::startup_sequence() }

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
    let mut pos3000: (u64,u64) = (1,1);
    while !inputs::handle_input(getch(), &mut data) {
        for i in 0..3000 {
            pos3000.0 = (pos3000.0 * 2791 + pos3000.1) % (screen_h+1) as u64;
            pos3000.1 = (pos3000.1 * 7129 + pos3000.0) % ((screen_w+1) as u64/4);
            print!("{}{}3000{}", Goto(pos3000.0 as u16, pos3000.1 as u16*4), C256((i%5) as u8+234), DefaultColor);
        }
        title_card.draw();
        println!("{}{}{}Insert 5¢ to begin{}", Down(3), Column((screen_w-18)/2), Blink, NotBlink);
    }
    data.draw();
    data.0.ctx = inputs::Context::Map;
    while !inputs::handle_input(getch(), &mut data) {        
        data.2 = &maps[data.0.current_map];
        data.draw();
        data.0.draw();
    }
}
