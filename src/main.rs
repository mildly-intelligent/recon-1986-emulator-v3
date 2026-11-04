mod inputs;
mod utils;
mod data;
mod runtime;

const SCALE: (u16,u16) = (5,3);
const MAP_ROWS: usize = 12;

fn main() {
    ncurses::noecho();
    ncurses::cbreak();

    let win = ncurses::initscr();

    ncurses::keypad(win, true);
    
    let (mut s, mut p, maps, (w, h)) = runtime::init(win).unwrap();
    let m = &maps[s.current_map];
    let mut data = data::Data(&mut s, &mut p, &m);

    runtime::start(&mut data, (w, h));

    while !inputs::handle_input(ncurses::getch(), &mut data) {        
        data.2 = &maps[data.0.current_map];
        data.draw();
        data.0.draw();
    }
}
