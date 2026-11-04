pub mod startup;

use ncurses::getch;

use crate::utils::*;
use crate::data::{Data, State, map::Map, player::Player};
use crate::inputs::{handle_input, Context};

pub fn init<'a>(win: *mut i8) -> Result<(State, Player, [Map;2], (usize, usize)), ()> {
    let state = State {
        win,
        current_map: 0,
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
            .set_interact(b'0', Some("Window"), "Look out window?", |d| Overlay::print_new_ctx(d, "It's just the vast expanse of the Indian Ocean"))
            .set_interact(b'1', Some("Pile of papers"), "Examine?", |d| Overlay::print_new_ctx(d, "Nothing of note, looks like someone dropped them in a hurry"))
            .set_interact(b'2', Some("Bed"), "Examine?", |d| Overlay::print_new_ctx(d, "Why you snooping?"))
            .dup_interact(b'4', b'2', None)?
            .dup_interact(b'5', b'2', None)?
            .set_interact(b'7', Some("Bed"), "Examine?", |d| Overlay::print_new_ctx(d, &format!("This is honestly concerning, there were four beds that belonged to people you may or may not know.\nFrom a gamer perspective I can get checking out one of them, maybe even two, but {}all four{}?\nThis is honestly disgusting behavior and you should be ashamed of yourself.", Italic, NotItalic)))
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
            .set_interact(b'1', Some("Table"), "Examine?", |d| Overlay::print_new_ctx(d, "There's just some science equipment, nothing you understand"))
            .set_interact(b'2', Some("Broken glass"), "Examine?", |d| Overlay::print_new_ctx(d, &format!("A puddle of mostly clear liquid.\nIn the puddle there's shards you see shards of a glass vial labeled {}Y-909{}\n{}{}What were you looking for again?{}{}", Bold, NotBold, Gray, Italic, NotItalic, DefaultColor))),
    ];

    let player = Player {
        pos: (3,17),
        interact: None
    };

    Ok((state, player, maps, (screen_w, screen_h)))
}

pub fn start(data: &mut Data, (w, h): (usize, usize)) {
    if !cfg!(debug_assertions) {startup::startup_sequence() }

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