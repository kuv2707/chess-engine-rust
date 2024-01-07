use crate::engine::{moves::find_in_raw_move_targets, parse_pos};

mod engine;
mod server;
fn dev() {
    //load env
    dotenv::dotenv().ok();

    let fenstr = std::env::var("FEN")
        .unwrap_or_else(|_| "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());
    let board = engine::board::create_board(&fenstr);
    // let pos=encode_pos(5, 1);
    // println!("pos:{}", pos_as_string(&pos));

    let pos = parse_pos("e4");

    println!(
        "{}",
        find_in_raw_move_targets(&board, &pos.unwrap(), &engine::piece::PieceColor::WHITE)
    );
}

fn main() {
    server::main();

    // dev();
    // return;
    // let mut board = engine::board::create_board("r4bn1/ppppp1pp/3p4/3r1k1n/Q7/4PP2/PPPP3P/RNB1KBbR w");
    // println!("{}", board);
    // //accept input
    // loop {
    //     let mut input = String::new();
    //     println!("Current turn: {}",board.side_to_move);
    //     println!("Enter move: ");
    //     std::io::stdin().read_line(&mut input).unwrap();
    //     if input.trim() == "exit" {
    //         break;
    //     }
    //     let m_res: Result<u16, String> = parse_move(&input.trim());
    //     if m_res.is_err() {
    //         println!("invalid move");
    //         continue;
    //     }
    //     let mov = m_res.unwrap();
    //     println!("move: {}", move_as_string(&mov));
    //     //todo: check if move is legal before making it
    //     board.make_move(mov);
    //     println!("{}", board);
    //     println!("Computer's turn");
    //     let (score, best_move) = board.best_move(4);
    //     board.make_move(best_move);
    //     println!("made move: {}", move_as_string(&best_move));
    //     println!("{}", board);
    // }
}
