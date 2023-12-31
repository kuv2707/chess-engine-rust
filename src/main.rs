

use crate::engine::{
    decode_move,
    moves::all_possible_valid_moves,
};

mod engine;

fn main() {
    //load env
    dotenv::dotenv().ok();
    let fenstr = std::env::var("FEN").unwrap_or_else(|_| "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());
    let board = engine::board::create_board(&fenstr);
    println!("{}\n\n Check?: {}", board, board.has_check(&board.side_to_move.opponent_color()));
    board.plot(
        all_possible_valid_moves(&board)
            .iter()
            .map(|m| decode_move(&m).1)
            .collect::<Vec<_>>(),
    );
    println!("board score: {}",board.evaluate());
    // board.plot(pawn_moves_raw(encode_pos(6, 3), &board));
}
