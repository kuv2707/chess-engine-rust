use std::fmt;


#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
}
#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}
#[derive(Copy, Clone, Debug)]
pub enum PieceColor {
    WHITE,
    BLACK,
}




impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece = match self.piece_type {
            super::piece::PieceType::PAWN => "P",
            super::piece::PieceType::KNIGHT => "N",
            super::piece::PieceType::BISHOP => "B",
            super::piece::PieceType::ROOK => "R",
            super::piece::PieceType::QUEEN => "Q",
            super::piece::PieceType::KING => "K",
        };
        let fen:String = match self.color {
            super::piece::PieceColor::WHITE => piece.to_uppercase(),
            super::piece::PieceColor::BLACK => piece.to_lowercase(),
        };
        write!(f, "{}", fen)
    }
}

impl Piece {
    pub fn new(color: PieceColor, piece_type: PieceType) -> Piece {
        Piece {
            color: color,
            piece_type: piece_type,
        }
    }
    // pub fn moves(&self, base: super::board::Position) -> Vec<super::board::Position> {
    //     match self.piece_type {
    //         PieceType::PAWN => pawn_moves(base, self.color),
    //         PieceType::KNIGHT => knight_moves(base),
    //         PieceType::BISHOP => bishop_moves(base),
    //         PieceType::ROOK => rook_moves(base),
    //         PieceType::QUEEN => queen_moves(base),
    //         PieceType::KING => king_moves(base),
    //     }
    // }
}