use engine::moves::knight_moves_raw;

use crate::engine::{
    board::encode_pos,
    decode_move,
    moves::{
        all_possible_raw_moves, all_possible_valid_moves, king_moves_raw, pawn_moves_raw,
        queen_moves_raw, rook_moves_raw, bishop_moves_raw,
    },
};

mod engine;

fn main() {
    //load env
    dotenv::dotenv().ok();
    let mut board = engine::board::create_board("rnbq1bnr/ppp1pppp/8/2kp4/5P2/8/PPPPPBPP/RN1QKBNR b");
    println!("{}", board);
    // println!("{:?}",board.has_check(&board.side_to_move));
    // board.side_to_move = board.side_to_move.opponent_color();
    board.plot(
        all_possible_valid_moves(&board)
            .iter()
            .map(|m| decode_move(&m).1)
            .collect::<Vec<_>>(),
    );
    // board.plot(pawn_moves_raw(encode_pos(6, 3), &board));
}
