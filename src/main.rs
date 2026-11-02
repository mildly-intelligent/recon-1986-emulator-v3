mod inputs;
mod map;
mod overlay;
mod player;
mod utils;

use crate::utils::*;
use {map::Map, overlay::Overlay, player::Player};

use ncurses::{cbreak, getch, initscr, keypad, noecho};

const SCALE: (u16,u16) = (5,3);
const MAP_ROWS: usize = 12;

struct State {
    win: *mut i8,
    current_map: usize,
    ctx: inputs::Context,
    current_objective: String,
}

fn ellipsis(ticks: usize) {
    print!("{}{}", Up(1), Save);
    for t in 0..ticks {
        let t = t%4;
        println!("{}{}", Restore, ".".repeat(t) + &" ".repeat(3-t));
        sleep(250);
    }
    println!("{}...", Restore);
}
fn startup_sequence() {
    let debug =   |msg: &str| format!("{}  {}{}  Debug{}{} {msg}", Column(1), Magenta,  Bold, NotBold, DefaultColor);
    let info =    |msg: &str| format!("{}  {}{}   Info{}{} {msg}", Column(1), Cyan,     Bold, NotBold, DefaultColor);
    let warning = |msg: &str| format!("{}  {}{}Warning{}{} {msg}", Column(1), Yellow,   Bold, NotBold, DefaultColor);
    let error =   |msg: &str| format!("{}  {}{}  Error{}{} {msg}", Column(1), Red,      Bold, NotBold, DefaultColor);

    // I have no idea if any of this is even remotely realistic
    println!("{}Thank you for using Recon-1986-emulator!", Column(1));
    println!("{}This is version 3 of the emulator, you can view the changelog at https://reconemulator.net/v3/changelog", Column(1));
    println!("{}Starting emulator with logging set to {}{}Debug{}{}", Column(1), Magenta, Bold, NotBold, DefaultColor);
    println!("{}", info("Initializing 4kB of virtual ram"));
    ellipsis(9);
    println!("{}", info("Loading program ROM"));
    ellipsis(8);
    println!("{}", warning("Map data may have been corrupted or lost when loading"));
    sleep(250);
    println!("{}", debug("Failed to write over data in ROM at address $3000-$39FF (map data)"));
    sleep(250);
    println!("{}", debug("Possible faulty initial RAM decay pattern"));
    sleep(250);
    println!("{}", info("Initializing virtual display"));
    ellipsis(6);
    println!("{}", info("Initializing virtual controls panel"));
    ellipsis(3);
    println!("{}", info("Initializing audio pipe"));
    ellipsis(5);
    println!("{}", error("Compatible audio driver not found, skipping"));
    println!("{}", info("Finishing setup"));
    ellipsis(9);
    println!("{}", error("Failed to purge God, skipping"));
    print!("{}{}", debug("3000"), Save);
    for _ in 0..3000 {
        println!("{}3000{}", Restore, Save);
        sleep_micros(50);
    }
}

fn draw(state: &mut State, player: &mut Player, map: &Map) {
    print!("{}{}", Clear, Home);
    player.get_interact(map);
    map.draw(state, player);
    player.draw(state, map);
    println!();
}

fn main() {
    noecho();
    cbreak();

    let mut state = State {
        win: initscr(),
        current_map: 0,
        ctx: inputs::Context::Title,
        current_objective: String::from("Explore"),
    };
    keypad(state.win, true);

    let screen_w = getmaxx(state.win) as u16;

    let maps: [Map; 2] = [
        Map::new([
            "              ---3---           ",
            "        ---00--     ------      ",
            "---------                |      ",
            "|                   1    +##### ",
            "---------------          |    # ",
            "              ---+--------    # ",
            "  -------------  #         #### ",
            "  |           |  #  ------ #    ",
            "  |           |  ###+    - #####",
            "  |           |  #  ---+--     #",
            "  --------+----  #     #       #",
            "          ########     #########",
        ])
            .set_interact(b'0', Some("Window"), "Look out window?", |state,_p,_m| {state.ctx=inputs::Context::Overlay; Overlay::print_new(state, "It's just the vast expanse of the Indian Ocean")})
            .set_interact(b'1', Some("Pile of papers"), "Examine?", |state,_p,_m| {state.ctx=inputs::Context::Overlay; Overlay::print_new(state, "Nothing of note, looks like someone dropped them in a hurry")})
            .set_interact(b'3', Some("Door"), "Go to Lab 1?", |state,p,_m| {state.current_map = 1; p.pos = (10,16)}),
        Map::new([
            "         ---------------        ",
            "         |             |        ",
            "         |    111      |        ",
            "         |             |        ",
            "         |             |        ",
            "         |---------    |        ",
            "         |             |        ",
            "         |         |   |        ",
            "         |    ------   |        ",
            "         |             |        ",
            "         |             |        ",
            "         ------000------        ",
        ])
            .set_interact(b'0', Some("Door"), "Go back main hull?", |state,p,_m| {state.current_map = 0; p.pos = (1,17);})
            .set_interact(b'1', Some("Table"), "Examine?", |state,_p,_m| {state.ctx=inputs::Context::Overlay; Overlay::print_new(state, "There's just some science equipment")}),
    ];

    let mut player = Player {
        pos: (3,17),
        interact: None
    };
    
    if !cfg!(debug_assertions) { startup_sequence() }

    println!("{}{}\x1b[?25l", Clear, Home);
    let title_card = overlay::Overlay::from(&state, &format!("  __ __       _____    _____     _____      __   __   
 /_/\\__/\\   /\\_____\\  /\\ __/\\   ) ___ (    /_/\\ /\\_\\  
 ) ) ) ) ) ( (_____/  ) )__\\/  / /\\_/\\ \\   ) ) \\ ( (  
/_/ /_/_/   \\ \\__\\   / / /    / /_/ (_\\ \\ /_/   \\ \\_\\ 
\\ \\ \\ \\ \\   / /__/_  \\ \\ \\_   \\ \\ )_/ / / \\ \\ \\   / / 
 )_) ) \\ \\ ( (_____\\  ) )__/\\  \\ \\/_\\/ /   )_) \\ (_(  
 \\_\\/ \\_\\/  \\/_____/  \\/___\\/   )_____(    \\_\\/ \\/_/  \n\n{}\n{}",
        pad(&String::from("Erimita Interactive LLC."), ' ', 54),
        pad(&String::from("c. 1986"), ' ', 54)));
    // while !inputs::handle_input(getch(), &mut state, None, None) {
        title_card.draw(&mut state);
        println!("{}{}{}Insert 5¢ to begin{}", Down(3), Column((screen_w-18)/2), Blink, NotBlink);
    // }
    let mut map = &maps[state.current_map];
    draw(&mut state, &mut player, &map);
    state.ctx = inputs::Context::Map;
    while !inputs::handle_input(getch(), &mut state, Some(&mut player), Some(&map)) {        
        map = &maps[state.current_map];
        draw(&mut state, &mut player, &map);
    }
}
