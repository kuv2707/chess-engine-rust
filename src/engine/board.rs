pub type Position = u8;
use std::fmt;

use super::piece::{Piece, PieceColor, PieceType};
#[derive(Debug)]
pub struct Board {
    pub squares: [Option<Piece>; 64],
    pub side_to_move: PieceColor,
    pub castling_rights: u8,
    pub en_passant_square: Option<Position>,
    pub halfmove_clock: u8,
    pub fullmove_number: u8,
}

impl Board {
    pub fn get_piece(&self, square: u8) -> Option<Piece> {
        self.squares[square as usize]
    }
    pub fn set_piece(&mut self, square: u8, piece: Option<Piece>) {
        self.squares[square as usize] = piece;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_string = String::new();
        for rank in 0..8 {
            for file in 0..8 {
                let piece = self.get_piece(rank * 8 + file);
                match piece {
                    Some(p) => board_string.push_str(&format!("{} ", p)),
                    None => board_string.push_str(".. "),
                }
            }
            board_string.push_str("\n");
        }
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
