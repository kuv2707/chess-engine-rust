mod engine;

fn main() {
    let board=engine::board::create_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    println!("{}",board);
    let best_move = engine::best_move(board);
    println!("best move: {:?}", best_move);
}
