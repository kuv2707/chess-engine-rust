use super::board::{decode_pos, encode_pos, Position};

//define macro to get nth bit as i8

macro_rules! nth_bit {
    ($n:expr, $bit:expr) => {
        ((($n) >> ($bit)) & 1) as i8
    };
}

macro_rules! in_bounds {
    ($r:expr, $f:expr) => {
        $r < 8 && $f < 8 && $r >= 0 && $f >= 0
    };
}

pub fn slant_moves_raw(base: Position) -> Vec<Position> {
    let mut moves: Vec<Position> = Vec::new();
    let (r, f) = decode_pos(base);
    let mut dir = 0;
    while dir <= 3 {
        let coeff_1 = nth_bit!(dir, 0) * 2 - 1;
        let coeff_2 = nth_bit!(dir, 1) * 2 - 1;
        let mut i = 1;
        loop {
            let newx: i16 = (r as i8 + i * coeff_1) as i16;
            let newy: i16 = (f as i8 + i * coeff_2) as i16;
            if !in_bounds!(newx, newy) {
                break;
            }
            let m = encode_pos(newx as u8, newy as u8);
            moves.push(m);
            i += 1;
        }
        dir += 1;
    }

    moves
}

pub fn rect_moves_raw(base: Position) -> Vec<Position> {
    let mut moves: Vec<Position> = Vec::new();
    let (r, f) = decode_pos(base);
    let coeffs=[(1,0),(0,1),(-1,0),(0,-1)];
    let mut dir:usize = 0;
    while dir <= 3 {
        let coeff_1 = coeffs[dir].0;
        let coeff_2 = coeffs[dir].1;
        let mut i = 1;
        loop {
            let newx: i16 = r as i16 + i * coeff_1;
            let newy: i16 = f as i16 + i * coeff_2;
            println!("{} {}", newx, newy);
            if !in_bounds!(newx, newy) {
                break;
            }
            let m = encode_pos(newx as u8, newy as u8);
            moves.push(m);
            i += 1;
        }
        dir += 1;
    }

    moves
}

pub fn knight_moves_raw(base: Position) -> Vec<Position> {
    let mut moves: Vec<Position> = Vec::new();
    let (r, f) = decode_pos(base);
    let coeffs=[(1,2),(2,1),(2,-1),(1,-2),(-1,-2),(-2,-1),(-2,1),(-1,2)];
    let mut dir:usize = 0;
    while dir <= 7 {
        let coeff_1 = coeffs[dir].0;
        let coeff_2 = coeffs[dir].1;
        let newx: i16 = r as i16 + coeff_1;
        let newy: i16 = f as i16 + coeff_2;
        if in_bounds!(newx, newy) {
            let m = encode_pos(newx as u8, newy as u8);
            moves.push(m);
        }
        dir += 1;
    }

    moves
}

pub fn king_moves_raw(base: Position) -> Vec<Position> {
    let mut moves: Vec<Position> = Vec::new();
    let (r, f) = decode_pos(base);
    let coeffs=[(1,0),(0,1),(-1,0),(0,-1),(1,1),(-1,-1),(-1,1),(1,-1)];
    let mut dir:usize = 0;
    while dir <= 7 {
        let coeff_1 = coeffs[dir].0;
        let coeff_2 = coeffs[dir].1;
        let newx: i16 = r as i16 + coeff_1;
        let newy: i16 = f as i16 + coeff_2;
        if in_bounds!(newx, newy) {
            let m = encode_pos(newx as u8, newy as u8);
            moves.push(m);
        }
        dir += 1;
    }

    moves
}

pub fn pawn_moves_raw(base: Position, color: i8) -> Vec<Position> {
    let mut moves: Vec<Position> = Vec::new();
    let (r, f) = decode_pos(base);
    let coeff:i16 = (color * 2 - 1) as i16;
    let newr: i16 = r as i16 + coeff as i16;
    let newf: i16 = f as i16;

    for element in [(newr+coeff,newf),(newr+coeff,newf+1),(newr+coeff,newf-1),(newr+coeff*2,newf)]{
        let rr: i16 = element.0;
        let ff: i16 = element.1;
        if in_bounds!(rr, ff) {
            let m = encode_pos(rr as u8, ff as u8);
            moves.push(m);
        }
    }

    moves
}

pub fn rook_moves_raw(base: Position) -> Vec<Position> {
    rect_moves_raw(base)
}
pub fn bishop_moves_raw(base: Position) -> Vec<Position> {
    slant_moves_raw(base)
}
pub fn queen_moves_raw(base: Position) -> Vec<Position> {
    let mut moves: Vec<Position> = Vec::new();
    moves.append(&mut slant_moves_raw(base));
    moves.append(&mut rect_moves_raw(base));
    moves
}