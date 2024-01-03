use crate::engine::{
    board::{encode_pos, pos_as_string}, decode_move, move_as_string, moves::{all_possible_valid_moves, find_in_raw_move_targets}, Move, parse_move,
};

mod engine;

fn dev() {
    //load env
    dotenv::dotenv().ok();

    let fenstr = std::env::var("FEjN")
        .unwrap_or_else(|_| "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());
    let board = engine::board::create_board(&fenstr);
    let pos=encode_pos(5, 1);
    println!("pos:{}", pos_as_string(&pos));
    println!("isthere:{}", find_in_raw_move_targets(&board,pos ));

}

fn main() {
    // dev();
    // return;
    let mut board = engine::board::create_board(
        "rnb1kbnr/pppppp1p/4q3/3Q4/7p/6B1/PPPPPPPP/RNB1K1NR b",
    );
    println!("{}", board);
    //accept input
    loop {
        let mut input = String::new();
        println!("Enter move: ");
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "exit" {
            break;
        }
        let m_res: Result<u16, String> = parse_move(&input.trim());
        if m_res.is_err() {
            println!("invalid move");
            continue;
        }
        let mov = m_res.unwrap();
        println!("move: {}",move_as_string(&mov));
        board.make_move(mov);
        println!("{}", board);
        println!("Computer's turn");
        let (score, best_move) = board.best_move(2);
        board.make_move(best_move);
        println!("{}",board);
    }
    

}