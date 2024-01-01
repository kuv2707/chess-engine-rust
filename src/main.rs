use crate::engine::{
    board::encode_pos, decode_move, move_as_string, moves::all_possible_valid_moves, Move, parse_move,
};

mod engine;

fn dev() {
    //load env
    dotenv::dotenv().ok();

    let fenstr = std::env::var("FEN")
        .unwrap_or_else(|_| "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string());
    let mut board = engine::board::create_board(&fenstr);

    let pmoves = all_possible_valid_moves(&mut board)
        .iter()
        .map(|m| decode_move(&m).1)
        .collect::<Vec<_>>();
    board.plot(pmoves);

    let (score, best_move) = board.best_move(4);
    println!(
        "best move: {:?} and board score: {}",
        move_as_string(&best_move),
        score
    );
    println!("for board\n{}", board);

    // println!("board score: {}",board.evaluate());
    // board.plot(pawn_moves_raw(encode_pos(6, 3), &board));

}

fn main() {
    let mut board = engine::board::create_board(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w",
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
        let (score, best_move) = board.best_move(4);
        board.make_move(best_move);
        println!("{}",board);
    }
    

}