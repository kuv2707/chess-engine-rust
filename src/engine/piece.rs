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
        let color = match self.color {
            super::piece::PieceColor::WHITE => "w",
            super::piece::PieceColor::BLACK => "b",
        };
        write!(f, "{}{}", color, piece)
    }
}