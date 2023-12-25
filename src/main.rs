use crate::engine::board::encode_pos;

mod engine;

fn main() {
    let board=engine::board::create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    println!("{}",board);
    let k=engine::moves::rect_moves(encode_pos(3, 3)).iter().map(|x| engine::board::decode_pos(*x)).collect::<Vec<(u8,u8)>>();
    println!("{:?}",k);
}
