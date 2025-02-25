use crate::{Board, CastleOpt, Color, Piece, SMove};
use wasm_bindgen::prelude::*;

const PAWN_SQUARE_RATINGS: [i32; 64] = [
    90, 90, 90, 90, 90, 90, 90, 90, 130, 130, 130, 130, 130, 130, 130, 130, 120, 120, 120, 120,
    120, 120, 120, 120, 110, 110, 110, 110, 110, 110, 110, 110, 90, 90, 90, 140, 140, 90, 90, 90,
    100, 90, 90, 100, 100, 90, 90, 100, 100, 100, 100, 80, 80, 100, 100, 100, 90, 90, 90, 90, 90,
    90, 90, 90,
];

const KNIGHT_SQUARE_RATINGS: [i32; 64] = [
    290, 290, 290, 290, 290, 290, 290, 290, 290, 300, 300, 300, 300, 300, 300, 290, 290, 300, 300,
    300, 300, 300, 300, 290, 290, 300, 300, 300, 300, 300, 300, 290, 290, 300, 300, 300, 300, 300,
    300, 290, 290, 300, 310, 300, 300, 310, 300, 290, 290, 300, 300, 310, 310, 300, 300, 290, 290,
    290, 290, 290, 290, 290, 290, 290,
];

const BISHOP_SQUARE_RATINGS: [i32; 64] = [
    300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300,
    300, 300, 300, 300, 300, 300, 310, 300, 300, 300, 300, 310, 300, 300, 300, 310, 300, 300, 310,
    300, 300, 300, 300, 300, 310, 310, 300, 300, 300, 300, 310, 300, 310, 310, 300, 310, 300, 300,
    300, 290, 300, 300, 290, 300, 300,
];

const ROOK_SQUARE_RATINGS: [i32; 64] = [
    500, 500, 500, 500, 500, 500, 500, 500, 510, 510, 510, 510, 510, 510, 510, 510, 500, 500, 500,
    500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500, 500,
    500, 500, 495, 500, 500, 500, 500, 500, 500, 495, 490, 500, 500, 500, 500, 500, 500, 490, 495,
    490, 500, 510, 510, 505, 490, 495,
];

const KING_SQUARE_RATINGS: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, -1, 0, 0, 0, 0, 0, 0, -1, -1, -1,
    0, 0,
];

fn rate_pos(board: &Board) -> i32 {
    let mut rating = 0;
    let pieces = [
        (board.white_pawns, &PAWN_SQUARE_RATINGS, true),
        (board.black_pawns, &PAWN_SQUARE_RATINGS, false),
        (board.white_knights, &KNIGHT_SQUARE_RATINGS, true),
        (board.black_knights, &KNIGHT_SQUARE_RATINGS, false),
        (board.white_bishops, &BISHOP_SQUARE_RATINGS, true),
        (board.black_bishops, &BISHOP_SQUARE_RATINGS, false),
        (board.white_rooks, &ROOK_SQUARE_RATINGS, true),
        (board.black_rooks, &ROOK_SQUARE_RATINGS, false),
        (board.white_kings, &KING_SQUARE_RATINGS, true),
        (board.black_kings, &KING_SQUARE_RATINGS, false),
    ];

    for (piece, rating_table, color) in pieces {
        let mut other = piece;
        while other != 0 {
            let square = other.trailing_zeros();
            other ^= 1 << square;
            if color {
                rating += rating_table[63 - square as usize];
            } else {
                rating -= rating_table[square as usize];
            }
        }
    }

    rating += (board.white_queens.count_ones() * 900) as i32;
    rating -= (board.black_queens.count_ones() * 900) as i32;

    return if board.turn == Color::WHITE {
        rating
    } else {
        -rating
    };
}

fn quiescence_search(board: &Board, _alpha: i32, _beta: i32) -> i32 {
    let original = rate_pos(board);
    let mut best = original;
    if original >= _beta {
        return original;
    }

    let mut alpha = if _alpha < original { original } else { _alpha };

    for pair in board.get_next_move_boards() {
        if pair.0.ep_move
            || match board.get_piece_at(pair.0.to) {
                (Piece::NONE, Color::WHITE) => false,
                (Piece::NONE, Color::BLACK) => false,
                _ => true,
            }
        {
            let rating = -quiescence_search(&pair.1, -_beta, alpha);
            if rating >= _beta {
                return rating;
            }
            if rating > best {
                best = rating;
            }
            if rating > alpha {
                alpha = rating;
            }
        }
    }

    return best;
}

fn alpha_beta(board: &Board, depth: i32, _alpha: i32, _beta: i32) -> i32 {
    let mut alpha = _alpha;
    if depth == 0 {
        return quiescence_search(board, alpha, _beta);
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
