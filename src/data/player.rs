use crate::utils::*;

use super::{State, map::Map};

#[allow(dead_code)]
pub enum MoveDir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Player {
    pub pos: (usize, usize),
    pub interact: Option<u8>,
}
impl Player {
    pub fn mov(&mut self, dir: MoveDir, map: &Map) {
        let pos_old = self.pos.clone();
        let c_old = map.at(self.pos).unwrap();
        self.pos = self.get(dir);
        if self.pos.0 as usize >= map.rows() {
            self.pos = pos_old;
            return;
        } else if self.pos.1 as usize >= map.cols() {
            self.pos = pos_old;
            return;
        }
        let c = map.at(self.pos).unwrap();
        if [b'-',b'|',b'?',b'X',b'0',b'1',b'2',b'3',b'4',b'5',b'6',b'7',b'8',b'9',b'A',b'B',b'C',b'D',b'E',b'F'].contains(&c) {
            self.pos = pos_old;
            return;
        }
        if c_old == b'#' && c == b' ' {
            self.pos = pos_old;
            return;
        }
    }

    pub fn get(&self, dir: MoveDir) -> (usize,usize) {
        match dir {
            MoveDir::Up =>    (self.pos.0-1, self.pos.1  ),
            MoveDir::Down =>  (self.pos.0+1, self.pos.1  ),
            MoveDir::Left =>  (self.pos.0,   self.pos.1-1),
            MoveDir::Right => (self.pos.0,   self.pos.1+1),
        }
    }

    pub fn draw(&self, state: &mut State, map: &Map) {
        let (w,h) = crate::SCALE;
        let x = (getmaxx(state.win) as usize - (map.cols()*w))/2;
        let y = (getmaxy(state.win) as usize - (map.rows()*h))/3;
        for i in 0..h {
            print!("{}{}", Goto(self.pos.0*h+y+i, self.pos.1*w+x), "█".repeat(w as usize));
        }
    }

    pub fn get_interact(&mut self, map: &Map) -> Option<(usize,usize)> {
        let pu = self.get(MoveDir::Up);
        let pd = self.get(MoveDir::Down);
        let pr = self.get(MoveDir::Right);
        let pl = self.get(MoveDir::Left);

        if let Some(cu) = map.at(pu) && cu.is_ascii_hexdigit() {
            self.interact = Some(cu);
            Some(pu)
        } else if let Some(cd) = map.at(pd) && cd.is_ascii_hexdigit() {
            self.interact = Some(cd);
            Some(pd)
        } else if let Some(cr) = map.at(pr) && cr.is_ascii_hexdigit() {
            self.interact = Some(cr);
            Some(pr)
        } else if let Some(cl) = map.at(pl) && cl.is_ascii_hexdigit() {
            self.interact = Some(cl);
            Some(pl)
        } else {
            self.interact = None;
            None
        }
    }
}