use std::fmt;

use super::{
    decode_move,
    moves::{all_possible_raw_moves, all_possible_valid_moves},
    piece::{Piece, PieceColor, PieceType},
    weights::{get_piece_weight, get_positional_weight},
    Move,
};
pub type Position = u8;

pub fn decode_pos(position: &Position) -> (i8, i8) {
    let rank = position / 8;
    let file = position % 8;
    (rank as i8, file as i8)
}
pub fn encode_pos(rank: u8, file: u8) -> Position {
    (rank * 8 + file) as Position
}

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub squares: [Option<Piece>; 64],
    pub side_to_move: PieceColor,
    pub castling_rights: u8,
    pub en_passant_square: Option<Position>,
    pub halfmove_clock: u8,  //ignoring this for now
    pub fullmove_number: u8, //ignoring this for now
}

impl Board {
    pub fn get_piece(&self, square: u8) -> Option<Piece> {
        self.squares[square as usize]
    }
    pub fn set_piece(&mut self, square: u8, piece: Option<Piece>) {
        self.squares[square as usize] = piece;
    }
    pub fn snapshot_extra_state(&self) -> (u8, Option<Position>) {
        (self.castling_rights, self.en_passant_square)
    }
    pub fn restore_extra_state(&mut self, state: (u8, Option<Position>)) {
        self.castling_rights = state.0;
        self.en_passant_square = state.1;
    }
    pub fn make_move(&mut self, m: Move) {
        let (from, to) = decode_move(&m);
        let piece = self.get_piece(from);
        if piece.is_none() {
            panic!("No piece at source square for move: {:?}", decode_move(&m));
        }
        self.set_piece(to, piece);
        self.set_piece(from, None);
        self.side_to_move = self.side_to_move.opponent_color();
        self.evaluate_extra_state(from, to, piece.unwrap());
    }
    pub fn unmake_move(&mut self, m: Move) {
        let (from, to) = decode_move(&m);
        let piece = self.get_piece(to);
        if piece.is_none() {
            panic!(
                "No piece at destination square for move: {:?}",
                decode_move(&m)
            );
        }
        self.set_piece(from, piece);
        self.set_piece(to, None);
        self.side_to_move = self.side_to_move.opponent_color();
    }
    pub fn evaluate_extra_state(
        &mut self,
        from: Position,
        to: Position,
        piece: Piece,
    ) -> (u8, Option<Position>) {
        //castling rights
        //en passant square
        (0, None) //todo:implement castling and enpassant, currently ignoring
    }
    // this function checks whether the current side to move has any moves which coincide with the king of the color `col`
    pub fn has_check(&self, col: &PieceColor) -> bool {
        //board MUST have both kings

        let k_option = self.squares.iter().position(|&x| {
            x.is_some() && x.unwrap().color == *col && x.unwrap().piece_type == PieceType::KING
        });
        if k_option.is_none() {
            panic!("King of color {:?} not found on board", col);
        }
        let k = k_option.unwrap() as Position;

        // println!("King position: {:?}", decode_pos(&k));

        let valid_moves = all_possible_raw_moves(self); //even moves which lead to the mover's side getting a check are valid if the mover can capture the king, but this should never happen
        // print_destinations(&valid_moves);
        for m in valid_moves {
            if decode_move(&m).1 == k {
                return true;
            }
        }
        return false;
    }
    pub fn evaluate(&self) -> f32 {
        //evaluation criteria:
        // location of pieces on board
        // danger to king
        // danger to other pieces === potential to capture other pieces: but weightage given should be lesser than the weightage given to the piece itself
        let mut score = 0.0;
        let mut pos = 0;
        for piece in self.squares.iter() {
            if piece.is_some() {
                let p = &piece.unwrap();
                let s= (get_positional_weight(pos, p) + get_piece_weight(p))
                    * piece.unwrap().get_color().get_value() as f32;
                score += s;
                println!("{}", s);
            }
            pos += 1;
        }
        for mv in all_possible_valid_moves(self).iter() {
            let tentative_piece = self.get_piece(decode_move(&mv).1);
            if tentative_piece.is_some() {
                score -= get_piece_weight(&tentative_piece.unwrap())
                    * tentative_piece.unwrap().get_color().get_value() as f32;
            }
        }
        score
    }
    pub fn plot(&self, positions: Vec<Position>) {
        println!(
            "Plotting positions: {:?}",
            positions.iter().map(|m| decode_pos(&m)).collect::<Vec<_>>()
        );
        let mut board_string = String::new();
        let files = "   a b c d e f g h\n";
        board_string.push_str(files);
        for rank in 0..8 {
            board_string.push_str(&format!("{}| ", 8 - rank));
            for file in 0..8 {
                // let piece = self.get_piece(rank * 8 + file);
                // match piece {
                //     Some(p) => board_string.push_str(&format!("{} ", p)),
                //     None => board_string.push_str("  "),
                // }
                if positions.contains(&(rank * 8 + file)) {
                    board_string.push_str("* ");
                } else {
                    board_string.push_str("  ");
                }
            }
            board_string.push_str("\n");
        }
        board_string.push_str(files);

        println!("{}", board_string);
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_string = String::new();
        let files = "   a b c d e f g h\n";
        board_string.push_str(files);
        let divider = "-".repeat(20) + "\n";
        board_string.push_str(divider.as_str());

        for rank in 0..8 {
            board_string.push_str(&format!("{}| ", 8 - rank));
            for file in 0..8 {
                let piece = self.get_piece(rank * 8 + file);
                match piece {
                    Some(p) => board_string.push_str(&format!("{} ", p)),
                    None => board_string.push_str("* "),
                }
            }
            board_string.push_str("\n");
        }
        board_string.push_str(divider.as_str());
        board_string.push_str(files);
        write!(f, "{}", board_string)
    }
}

//"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
pub fn create_board(fen: &str) -> Board {
    let mut board = Board {
        squares: [None; 64],
        side_to_move: PieceColor::WHITE,
        castling_rights: 0b1111,
        en_passant_square: None,
        halfmove_clock: 0,
        fullmove_number: 0,
    };
    let info_array: Vec<&str> = fen.split(" ").collect();
    populate_pieces(&mut board, info_array[0]);
    for i in 1..info_array.len() {
        match i {
            1 => {
                if info_array[i] == "w" {
                    board.side_to_move = PieceColor::WHITE;
                } else {
                    board.side_to_move = PieceColor::BLACK;
                }
            }
            2 => {
                for c in info_array[i].chars() {
                    match c {
                        'K' => board.castling_rights |= 0b0001,
                        'Q' => board.castling_rights |= 0b0010,
                        'k' => board.castling_rights |= 0b0100,
                        'q' => board.castling_rights |= 0b1000,
                        _ => (),
                    }
                }
            }
            3 => {
                if info_array[i] == "-" {
                    board.en_passant_square = None;
                } else {
                    let file = info_array[i].chars().nth(0).unwrap() as u8 - 'a' as u8;
                    let rank = info_array[i].chars().nth(1).unwrap() as u8 - '1' as u8;
                    board.en_passant_square = Some(rank * 8 + file as Position);
                }
            }
            4 => board.halfmove_clock = info_array[i].parse::<u8>().unwrap(),
            5 => board.fullmove_number = info_array[i].parse::<u8>().unwrap(),
            _ => println!("Error parsing FEN string"),
        }
    }

    board
}

pub fn populate_pieces(board: &mut Board, piece_placement: &str) {
    let ranks: Vec<&str> = piece_placement.split("/").collect();
    let mut rank = 0;
    for r in ranks {
        let mut file = 0;
        for c in r.chars() {
            if c.is_digit(10) {
                file += c.to_digit(10).unwrap() as usize;
            } else {
                let piece = match c {
                    'p' => Some(Piece {
                        color: PieceColor::BLACK,
                        piece_type: PieceType::PAWN,
                    }),
                    'n' => Some(Piece {
                        color: PieceColor::BLACK,
                        piece_type: PieceType::KNIGHT,
                    }),
                    'b' => Some(Piece {
                        color: PieceColor::BLACK,
                        piece_type: PieceType::BISHOP,
                    }),
                    'r' => Some(Piece {
                        color: PieceColor::BLACK,
                        piece_type: PieceType::ROOK,
                    }),
                    'q' => Some(Piece {
                        color: PieceColor::BLACK,
                        piece_type: PieceType::QUEEN,
                    }),
                    'k' => Some(Piece {
                        color: PieceColor::BLACK,
                        piece_type: PieceType::KING,
                    }),
                    'P' => Some(Piece {
                        color: PieceColor::WHITE,
                        piece_type: PieceType::PAWN,
                    }),
                    'N' => Some(Piece {
                        color: PieceColor::WHITE,
                        piece_type: PieceType::KNIGHT,
                    }),
                    'B' => Some(Piece {
                        color: PieceColor::WHITE,
                        piece_type: PieceType::BISHOP,
                    }),
                    'R' => Some(Piece {
                        color: PieceColor::WHITE,
                        piece_type: PieceType::ROOK,
                    }),
                    'Q' => Some(Piece {
                        color: PieceColor::WHITE,
                        piece_type: PieceType::QUEEN,
                    }),
                    'K' => Some(Piece {
                        color: PieceColor::WHITE,
                        piece_type: PieceType::KING,
                    }),
                    _ => None,
                };
                board.set_piece(rank * 8 + file as Position, piece);
                file += 1;
            }
        }
        rank += 1;
    }
}

pub fn print_destinations(moves: &Vec<Move>) {
    println!(
        "Possible moves: {:?}",
        moves
            .iter()
            .map(|m| decode_move(&m).1)
            .map(|n| decode_pos(&n))
            .collect::<Vec<_>>()
    );
}
