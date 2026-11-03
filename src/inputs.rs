
use crate::player::MoveDir;

use ncurses::{KEY_BACKSPACE, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP};

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Context {
    Map,
    Overlay,
    Pause,
    Title,
}

fn do_interact(data: &mut crate::Data) {
    if let Some(i) = data.1.interact {
        (data.2.get_interact(i).unwrap().action)(data)
    }
}

pub fn handle_input(key: i32, data: &mut crate::Data) -> bool {
    match data.0.ctx {
        Context::Map => match key {
            KEY_UP => data.1.mov(MoveDir::Up, data.2),
            KEY_DOWN => data.1.mov(MoveDir::Down, data.2),
            KEY_LEFT => data.1.mov(MoveDir::Left, data.2),
            KEY_RIGHT => data.1.mov(MoveDir::Right, data.2),
            10 => do_interact(data),
            KEY_BACKSPACE => return true,
            _ => (),
        },
        Context::Overlay => match key {
            10 => {
                data.0.ctx = Context::Map;
                return true
            },
            _ => ()
        },
        Context::Pause => todo!(),
        Context::Title => match key {
            10 => return true,
            _ => (),
        },
    };
    false
}