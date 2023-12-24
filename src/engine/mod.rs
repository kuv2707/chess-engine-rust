use std::fmt;
use self::board::Board;

pub mod piece;
pub mod board;

#[derive(Copy, Clone, Debug)]
pub struct Move {
    from: u8,
    to: u8,

}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.from, self.to)
    }
}



pub fn best_move(board: Board) -> Move {
    Move {
        from: 0,
        to: 0,
    }
}

