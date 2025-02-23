use crate::{Board, CastleOpt, Color, Piece, SMove};
use wasm_bindgen::prelude::*;

fn rate_pos(board: &Board) -> i32 {
    return board.white_pawns.count_ones() as i32
        + (board.white_knights.count_ones() * 3) as i32
        + (board.white_bishops.count_ones() * 3) as i32
        + (board.white_rooks.count_ones() * 5) as i32
        + (board.white_queens.count_ones() * 9) as i32
        - board.black_pawns.count_ones() as i32
        - (board.black_knights.count_ones() * 3) as i32
        - (board.black_bishops.count_ones() * 3) as i32
        - (board.black_rooks.count_ones() * 5) as i32
        - (board.black_queens.count_ones() * 9) as i32;
}

fn alpha_beta(board: &Board, depth: i32, _alpha: i32, _beta: i32) -> i32 {
    let mut alpha = _alpha;
    if depth == 0 {
        if board.turn == Color::WHITE {
            return rate_pos(board);
        } else {
            return -rate_pos(board);
        }
    }

    let mut best = i32::MIN;
    for b in board.get_next_boards() {
        let rating = -alpha_beta(&b, depth - 1, -_beta, -alpha);
        if rating > best {
            best = rating;
            if rating > _alpha {
                alpha = rating;
            }
        }
        if rating >= _beta {
            return best;
        }
    }

    return best;
}

#[wasm_bindgen]
pub fn get_best_move(board: &Board, depth: i32) -> SMove {
    let mut best_score = i32::MIN;
    let mut best_move: SMove = SMove {
        from: 0,
        to: 0,
        promote_piece: Piece::NONE,
        castle_move: CastleOpt::NONE,
        ep_move: false,
    };
    for pair in board.get_next_move_boards() {
        let rating = -alpha_beta(&pair.1, depth, i32::MIN, i32::MAX);
        if rating > best_score {
            best_score = rating;
            best_move = pair.0;
        }
    }
    return best_move;
}
