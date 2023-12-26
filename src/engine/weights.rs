
use super::board::{decode_pos, Position};


const KING_WEIGHT: f32 = 200.0;
const QUEEN_WEIGHT: f32 = 9.0;
const ROOK_WEIGHT: f32 = 5.0;
const BISHOP_WEIGHT: f32 = 3.0;
const KNIGHT_WEIGHT: f32 = 3.0;
const PAWN_WEIGHT: f32 = 1.0;


//knight should as close to center as possible
pub fn knight_pos_wt(pos: Position) -> f32 {
    let (r, f) = decode_pos(pos);
    return 1.0 / (1.0 + (r - 3).abs() as f32 + (f - 3).abs() as f32);
}
//king should be as far from center as possible
pub fn king_pos_wt(pos: Position) -> f32 {
    let (r, f) = decode_pos(pos);
    return -1.0 / (1.0 + (r - 3).abs() as f32 + (f - 3).abs() as f32);
}
//pawn should be as far from center as possible
pub fn pawn_pos_wt(pos: Position) -> f32 {
    let (r, f) = decode_pos(pos);
    return -1.0 / (1.0 + (r - 3).abs() as f32 + (f - 3).abs() as f32);
}
//bishop should be as close to center as possible
pub fn bishop_pos_wt(pos: Position) -> f32 {
    let (r, f) = decode_pos(pos);
    return 1.0 / (1.0 + (r - 3).abs() as f32 + (f - 3).abs() as f32);
}
//rook should be as close to center as possible
pub fn rook_pos_wt(pos: Position) -> f32 {
    let (r, f) = decode_pos(pos);
    return 1.0 / (1.0 + (r - 3).abs() as f32 + (f - 3).abs() as f32);
}
//queen should be as close to center as possible
pub fn queen_pos_wt(pos: Position) -> f32 {
    let (r, f) = decode_pos(pos);
    return 1.0 / (1.0 + (r - 3).abs() as f32 + (f - 3).abs() as f32);
}
