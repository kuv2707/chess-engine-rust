use crate::engine::{
    board::encode_pos,
    decode_move, move_as_string,
    moves::all_possible_valid_moves,
};

mod engine;

fn main() {
    //load env
    dotenv::dotenv().ok();
    let fenstr = std::env::var("FEN")
        .unwrap_or_else(|_| "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());
    let mut board = engine::board::create_board(&fenstr);
    println!("-->{:?}", board.get_piece(encode_pos(2, 3)));
    println!(
        "{}\n\n Check?: {}",
        board,
        board.has_check(&board.side_to_move.opponent_color())
    );

    let pmoves = all_possible_valid_moves(&mut board)
        .iter()
        .map(|m| decode_move(&m).1)
        .collect::<Vec<_>>();
    board.plot(pmoves);
    println!("best move: {:?}", move_as_string(&board.best_move(3)));
    println!("for board\n {}", board);

    // println!("board score: {}",board.evaluate());
    // board.plot(pawn_moves_raw(encode_pos(6, 3), &board));
}
