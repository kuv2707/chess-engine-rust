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

pub fn slant_moves(base: Position) -> Vec<Position> {
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

pub fn rect_moves(base: Position) -> Vec<Position> {
    let mut moves: Vec<Position> = Vec::new();
    let (r, f) = decode_pos(base);
    let mut dir = 0;
    while dir <= 3 {
        let coeff_1 = nth_bit!(dir, 0) as i16 * 2 - 1;
        let coeff_2 = nth_bit!(dir, 1) as i16 * 2 - 1;
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
