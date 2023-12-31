use self::board::Position;

pub mod board;
pub mod moves;
pub mod piece;
pub mod weights;


pub type Move= u16;


/*
@returns a tuple of (from,to)    
 */
pub fn decode_move(m: &Move) -> (Position, Position) {
    let from = (m >> 6) as u8;
    let to = (m & 0b111111) as u8;
    (from, to)
}
pub fn encode_move(from: Position, to: Position) -> Move {
    ((from as u16) << 6) | (to as u16)
}


