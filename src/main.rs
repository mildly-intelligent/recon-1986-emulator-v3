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
    inventory: Vec<String>,
}
impl State {
    fn draw(&mut self) {
        let (screen_w, screen_h) = (getmaxx(self.win) as u16, getmaxy(self.win) as u16);

        let x = screen_w / 16;
        let state_y = (screen_h - (crate::MAP_ROWS as u16*crate::SCALE.1))/3 + crate::SCALE.1*12;
        let y = (state_y + screen_h) / 2;
        let w = screen_w * 1/8;
        println!("{}{}", Goto(state_y+1, x), "~".repeat(w as usize));
        println!("{}{}", Goto(y-1, x), pad(&format!("{}Current objective{}", Bold, NotBold), ' ', w as usize));
        println!("{}{}", Goto(y+1, x), pad(&self.current_objective, ' ', w as usize));
        println!("{}{}", Goto(screen_h as u16 - 1, x), "~".repeat(w as usize));

        println!("{}{}", Goto(state_y+1, x*12), "~".repeat(w as usize*2));
        println!("{}{}", Goto(y-1, x*12), pad(&format!("{}Inventory{}", Bold, NotBold), ' ', w as usize*2));
        println!("{}{}", Goto(y+1, x*12), pad(&self.inventory.join(" | "), ' ', w as usize*2));
        println!("{}{}", Goto(screen_h as u16 - 1, x*12), "~".repeat(w as usize*2));
    }
}
struct Data<'a>(&'a mut State, &'a mut Player, &'a Map);

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
    println!("{}", debug("Failed to write over data in ROM at address $2000-$2FFF (map data)"));
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
    println!("{}", error("Failed to read byte $3000 (forgotten), skipping"));
    print!("{}{}", debug("3000"), Save);
    for _ in 0..3000 {
        println!("{}3000{}", Restore, Save);
        sleep_micros(50);
    }
}

fn draw(Data(state, player, map): &mut Data) {
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

    let mut data = Data(&mut state, &mut player, &map);
    

    if !cfg!(debug_assertions) { startup_sequence() }

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
    draw(&mut data);
    data.0.ctx = inputs::Context::Map;
    while !inputs::handle_input(getch(), &mut data) {        
        data.2 = &maps[data.0.current_map];
        draw(&mut data);
        data.0.draw();
    }
}
