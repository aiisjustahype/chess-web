use core::panic;
use wasm_bindgen::prelude::*;

pub const FILE_A: u64 = 0x8080808080808080;
pub const FILE_B: u64 = 0x4040404040404040;
pub const FILE_C: u64 = 0x2020202020202020;
pub const FILE_D: u64 = 0x1010101010101010;
pub const FILE_E: u64 = 0x0808080808080808;
pub const FILE_F: u64 = 0x0404040404040404;
pub const FILE_G: u64 = 0x0202020202020202;
pub const FILE_H: u64 = 0x0101010101010101;

pub const RANK_1: u64 = 0x00000000000000FF;
pub const RANK_2: u64 = 0x000000000000FF00;
pub const RANK_3: u64 = 0x0000000000FF0000;
pub const RANK_4: u64 = 0x00000000FF000000;
pub const RANK_5: u64 = 0x000000FF00000000;
pub const RANK_6: u64 = 0x0000FF0000000000;
pub const RANK_7: u64 = 0x00FF000000000000;
pub const RANK_8: u64 = 0xFF00000000000000;

const CASTLE_WS_PATH: u64 = 0x000000000000000E;
const CASTLE_WL_PATH: u64 = 0x0000000000000038;
const CASTLE_BS_PATH: u64 = 0x0E00000000000000;
const CASTLE_BL_PATH: u64 = 0x3800000000000000;

const CASTLE_WS_EMPTY: u64 = 0x0000000000000006;
const CASTLE_WL_EMPTY: u64 = 0x0000000000000070;
const CASTLE_BS_EMPTY: u64 = 0x0600000000000000;
const CASTLE_BL_EMPTY: u64 = 0x7000000000000000;

const A1: u64 = FILE_A & RANK_1;
const C1: u64 = FILE_C & RANK_1;
const D1: u64 = FILE_D & RANK_1;
const F1: u64 = FILE_F & RANK_1;
const G1: u64 = FILE_G & RANK_1;
const H1: u64 = FILE_H & RANK_1;

const A8: u64 = FILE_A & RANK_8;
const C8: u64 = FILE_C & RANK_8;
const D8: u64 = FILE_D & RANK_8;
const F8: u64 = FILE_F & RANK_8;
const G8: u64 = FILE_G & RANK_8;
const H8: u64 = FILE_H & RANK_8;

const E1: u64 = FILE_E & RANK_1;
const E8: u64 = FILE_E & RANK_8;

const fn rays(dir: u8, sign: bool, mask: u64) -> [u64; 64] {
    let mut bitboards: [u64; 64] = [0; 64];

    let mut i = 0;

    while i < 64 {
        let mut point: u64 = 1 << i;
        let mut bitboard: u64 = 0;
        while point != 0 {
            point = if sign { point << dir } else { point >> dir };
            if mask != 0 {
                point = point & mask;
            }
            bitboard |= point;
        }
        bitboards[i] = bitboard;
        i += 1;
    }

    return bitboards;
}

const fn short_rays(dir: u8, sign: bool, mask: u64) -> [[u64; 64]; 64] {
    let mut bitboards: [[u64; 64]; 64] = [[0; 64]; 64];

    let mut i = 0;

    while i < 64 {
        let mut j = 0;

        while j < 64 {
            let stop: u64 = 1 << j;

            let mut point: u64 = 1 << i;
            let mut bitboard: u64 = 0;
            while point != 0 {
                point = if sign { point << dir } else { point >> dir };
                if mask != 0 {
                    point = point & mask;
                }
                bitboard |= point;

                if point & stop != 0 {
                    break;
                }
            }
            bitboards[i][j] = bitboard;
            j += 1;
        }
        i += 1;
    }

    return bitboards;
}

const fn knight_comp() -> [u64; 64] {
    let mut bitboards: [u64; 64] = [0; 64];
    let mut i = 0;
    while i < 64 {
        let square = 1 << i;
        let to_squares: u64 = ({ ((square << 7) & !FILE_A) << 8 })
            | ((square << 9) & !FILE_H) << 8
            | (((square << 7) & !FILE_A) >> 1) & !FILE_A
            | (((square >> 9) & !FILE_A) >> 1) & !FILE_A
            | (((square << 9) & !FILE_H) << 1) & !FILE_H
            | (((square >> 7) & !FILE_H) << 1) & !FILE_H
            | ((square >> 7) & !FILE_H) >> 8
            | ((square >> 9) & !FILE_A) >> 8;

        bitboards[i] = to_squares;
        i += 1;
    }

    bitboards
}

const fn king_comp() -> [u64; 64] {
    let mut bitboards: [u64; 64] = [0; 64];
    let mut i = 0;
    while i < 64 {
        let square = 1 << i;
        bitboards[i] = square << 8
            | (square << 9) & !FILE_H
            | (square << 7) & !FILE_A
            | (square << 1) & !FILE_H
            | (square >> 1) & !FILE_A
            | (square >> 7) & !FILE_H
            | (square >> 9) & !FILE_A
            | square >> 8;
        i += 1;
    }
    bitboards
}

const RAY_N: [u64; 64] = rays(8, true, 0);
const RAY_S: [u64; 64] = rays(8, false, 0);
const RAY_W: [u64; 64] = rays(1, true, !FILE_H);
const RAY_E: [u64; 64] = rays(1, false, !FILE_A);
const RAY_NW: [u64; 64] = rays(9, true, !FILE_H);
const RAY_NE: [u64; 64] = rays(7, true, !FILE_A);
const RAY_SW: [u64; 64] = rays(7, false, !FILE_H);
const RAY_SE: [u64; 64] = rays(9, false, !FILE_A);

const SHORT_RAY_N: [[u64; 64]; 64] = short_rays(8, true, 0);
const SHORT_RAY_S: [[u64; 64]; 64] = short_rays(8, false, 0);
const SHORT_RAY_W: [[u64; 64]; 64] = short_rays(1, true, !FILE_H);
const SHORT_RAY_E: [[u64; 64]; 64] = short_rays(1, false, !FILE_A);
const SHORT_RAY_NW: [[u64; 64]; 64] = short_rays(9, true, !FILE_H);
const SHORT_RAY_NE: [[u64; 64]; 64] = short_rays(7, true, !FILE_A);
const SHORT_RAY_SW: [[u64; 64]; 64] = short_rays(7, false, !FILE_H);
const SHORT_RAY_SE: [[u64; 64]; 64] = short_rays(9, false, !FILE_A);

const KNIGHT_MOVES: [u64; 64] = knight_comp();
const KING_MOVES: [u64; 64] = king_comp();

fn get_ray(dir: &Dir) -> &[u64; 64] {
    match dir {
        Dir::N => &RAY_N,
        Dir::S => &RAY_S,
        Dir::E => &RAY_E,
        Dir::W => &RAY_W,
        Dir::NE => &RAY_NE,
        Dir::NW => &RAY_NW,
        Dir::SE => &RAY_SE,
        Dir::SW => &RAY_SW,
    }
}

fn get_short_ray(dir: &Dir) -> &[[u64; 64]; 64] {
    match dir {
        Dir::N => &SHORT_RAY_N,
        Dir::S => &SHORT_RAY_S,
        Dir::E => &SHORT_RAY_E,
        Dir::W => &SHORT_RAY_W,
        Dir::NE => &SHORT_RAY_NE,
        Dir::NW => &SHORT_RAY_NW,
        Dir::SE => &SHORT_RAY_SE,
        Dir::SW => &SHORT_RAY_SW,
    }
}

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy)]
pub enum Color {
    WHITE,
    BLACK,
}

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy)]
pub enum Piece {
    NONE,
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}

const PROMOTE_PIECES: [Piece; 4] = [Piece::KNIGHT, Piece::BISHOP, Piece::ROOK, Piece::QUEEN];

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum CastleOpt {
    NONE,
    WHITESHORT,
    BLACKSHORT,
    WHITELONG,
    BLACKLONG,
}

enum Dir {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

#[derive(Clone, Copy, PartialEq)]
enum Outcome {
    ONGOING,
    WHITE,
    BLACK,
    DRAW,
}

struct Move {
    pub from: u64,
    pub to: u64,
    pub promote_move: bool,
    pub castle_move: CastleOpt,
    pub ep_move: bool,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct SMove {
    pub from: u8,
    pub to: u8,
    pub promote_piece: Piece,
    pub castle_move: CastleOpt,
    pub ep_move: bool,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Board {
    pub turn: Color,
    white_pawns: u64,
    black_pawns: u64,
    white_knights: u64,
    black_knights: u64,
    white_bishops: u64,
    black_bishops: u64,
    white_rooks: u64,
    black_rooks: u64,
    white_queens: u64,
    black_queens: u64,
    white_kings: u64,
    black_kings: u64,
    castle_white_short: bool,
    castle_white_long: bool,
    castle_black_short: bool,
    castle_black_long: bool,
    last_pos_ep: u64,
    black_occupied: u64,
    white_occupied: u64,
    previous_positions: [[u64; 12]; 100],
    next_index: usize,
    outcome: Outcome,
}

#[wasm_bindgen]
impl Move {
    fn get_moves(&self) -> Vec<SMove> {
        if self.castle_move != CastleOpt::NONE {
            return vec![SMove {
                from: self.from.trailing_zeros() as u8,
                to: self.to.trailing_zeros() as u8,
                promote_piece: Piece::NONE,
                castle_move: self.castle_move,
                ep_move: false,
            }];
        }

        let mut moves: Vec<SMove> = Vec::new();
        let mut to_squares = self.to;
        while to_squares != 0 {
            let square = to_squares & (1 << to_squares.trailing_zeros());
            to_squares &= !square;

            if self.promote_move {
                for piece in PROMOTE_PIECES {
                    moves.push(SMove {
                        from: self.from.trailing_zeros() as u8,
                        to: square.trailing_zeros() as u8,
                        promote_piece: piece,
                        castle_move: CastleOpt::NONE,
                        ep_move: false,
                    });
                }
            } else {
                moves.push(SMove {
                    from: self.from.trailing_zeros() as u8,
                    to: square.trailing_zeros() as u8,
                    promote_piece: Piece::NONE,
                    castle_move: CastleOpt::NONE,
                    ep_move: self.ep_move,
                });
            }
        }
        return moves;
    }
}

#[wasm_bindgen]
impl SMove {
    pub fn print(&self) {
        println!("------------");
        match self.castle_move {
            CastleOpt::BLACKLONG => println!("black long castling"),
            CastleOpt::WHITELONG => println!("white long casling"),
            CastleOpt::BLACKSHORT => println!("black short castling"),
            CastleOpt::WHITESHORT => println!("white short casling"),
            CastleOpt::NONE => {
                println!("from: {}", self.from);
                println!("to: {}", self.to);
                if self.ep_move {
                    println!("en passant");
                }
                match self.promote_piece {
                    Piece::KING => println!("promote to king??"),
                    Piece::QUEEN => println!("promote to queen"),
                    Piece::ROOK => println!("promote to rook"),
                    Piece::BISHOP => println!("promote to bishop"),
                    Piece::KNIGHT => println!("promote to knight"),
                    Piece::PAWN => println!("promote to pawn??"),
                    Piece::NONE => {}
                }
            }
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        let pawns = 0b11111111_00000000;
        let knights = 0b01000010;
        let bishops = 0b00100100;
        let rooks = 0b10000001;
        let queens = 0b00010000;
        let kings = 0b00001000;

        let mut board = Self {
            turn: Color::WHITE,
            white_pawns: pawns,
            black_pawns: pawns.reverse_bits(),
            white_knights: knights,
            black_knights: knights.reverse_bits(),
            white_bishops: bishops,
            black_bishops: bishops.reverse_bits(),
            white_rooks: rooks,
            black_rooks: rooks.reverse_bits(),
            white_queens: queens,
            black_queens: queens << 8 * 7,
            white_kings: kings,
            black_kings: kings << 8 * 7,
            castle_white_short: true,
            castle_white_long: true,
            castle_black_short: true,
            castle_black_long: true,
            last_pos_ep: 0,
            black_occupied: 0,
            white_occupied: 0,
            previous_positions: [[0; 12]; 100],
            next_index: 0,
            outcome: Outcome::ONGOING,
        };

        board.white_occupied = board._white_occupied();
        board.black_occupied = board._black_occupied();

        board
    }
}

impl std::str::FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<&str>>();

        if parts.len() != 6 {
            return Err("invalid number of arguments".to_string());
        }

        let mut board = Board::default();

        if parts[1] == "b" {
            board.turn = Color::BLACK;
        }

        if !parts[2].contains("K") {
            board.castle_white_short = false;
        }
        if !parts[2].contains("Q") {
            board.castle_white_long = false;
        }
        if !parts[2].contains("k") {
            board.castle_black_short = false;
        }
        if !parts[2].contains("q") {
            board.castle_black_long = false;
        }

        if parts[3] != "-" {
            board.last_pos_ep = match parts[3].chars().nth(0).unwrap() {
                'a' => FILE_A,
                'b' => FILE_B,
                'c' => FILE_C,
                'd' => FILE_D,
                'e' => FILE_E,
                'f' => FILE_F,
                'g' => FILE_G,
                'h' => FILE_H,
                _ => return Err("invalid ep square".to_string()),
            } & match parts[3].chars().nth(1).unwrap() {
                '1' => RANK_1,
                '2' => RANK_2,
                '3' => RANK_3,
                '4' => RANK_4,
                '5' => RANK_5,
                '6' => RANK_6,
                '7' => RANK_7,
                '8' => RANK_8,
                _ => return Err("invalid ep square".to_string()),
            };
        }

        board.white_pawns = 0;
        board.black_pawns = 0;
        board.white_knights = 0;
        board.black_knights = 0;
        board.white_bishops = 0;
        board.black_bishops = 0;
        board.white_rooks = 0;
        board.black_rooks = 0;
        board.white_queens = 0;
        board.black_queens = 0;
        board.white_kings = 0;
        board.black_kings = 0;

        let rows = parts[0].split("/").collect::<Vec<&str>>();
        if rows.len() != 8 {
            return Err("invalid rank count".to_string());
        }

        for (y, row) in rows.into_iter().enumerate() {
            let mut x = 7;
            for c in row.chars() {
                match c {
                    '1' => {}
                    '2' => x -= 1,
                    '3' => x -= 2,
                    '4' => x -= 3,
                    '5' => x -= 4,
                    '6' => x -= 5,
                    '7' => x -= 6,
                    '8' => {}
                    'p' => board.black_pawns |= 1 << (7 - y) * 8 + x,
                    'n' => board.black_knights |= 1 << (7 - y) * 8 + x,
                    'b' => board.black_bishops |= 1 << (7 - y) * 8 + x,
                    'r' => board.black_rooks |= 1 << (7 - y) * 8 + x,
                    'q' => board.black_queens |= 1 << (7 - y) * 8 + x,
                    'k' => board.black_kings |= 1 << (7 - y) * 8 + x,
                    'P' => board.white_pawns |= 1 << (7 - y) * 8 + x,
                    'N' => board.white_knights |= 1 << (7 - y) * 8 + x,
                    'B' => board.white_bishops |= 1 << (7 - y) * 8 + x,
                    'R' => board.white_rooks |= 1 << (7 - y) * 8 + x,
                    'Q' => board.white_queens |= 1 << (7 - y) * 8 + x,
                    'K' => board.white_kings |= 1 << (7 - y) * 8 + x,
                    _ => return Err("invalid position".to_string()),
                }
                if x > 0 {
                    x -= 1;
                }
            }
        }

        return Ok(board);
    }
}

fn move_n(x: u64) -> u64 {
    x << 8
}
fn move_s(x: u64) -> u64 {
    x >> 8
}
fn move_w(x: u64) -> u64 {
    (x << 1) & !FILE_H
}
fn move_e(x: u64) -> u64 {
    (x >> 1) & !FILE_A
}
fn move_nw(x: u64) -> u64 {
    (x << 9) & !FILE_H
}
fn move_ne(x: u64) -> u64 {
    (x << 7) & !FILE_A
}
fn move_sw(x: u64) -> u64 {
    (x >> 7) & !FILE_H
}
fn move_se(x: u64) -> u64 {
    (x >> 9) & !FILE_A
}

pub fn print_bitboard(bitboard: u64) {
    for y in (0..8).rev() {
        for x in (0..8).rev() {
            let square = 1 << y * 8 + x;
            if square & bitboard == 0 {
                print!(" 0");
            } else {
                print!(" 1");
            }
        }
        print!("\n");
    }
}

#[wasm_bindgen]
pub struct MoveBoardPair(pub SMove, pub Board);

#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_clone(&self) -> Self {
        self.clone()
    }

    pub fn fen(&self) -> String {
        let mut rows: Vec<String> = Vec::new();
        for y in (0..8).rev() {
            let mut nothing = 0;
            let mut row = String::new();
            for x in (0..8).rev() {
                match self.get_piece_at(y * 8 + x) {
                    (Piece::NONE, Color::WHITE) => nothing += 1,
                    (Piece::NONE, Color::BLACK) => nothing += 1,
                    (piece, color) => {
                        if nothing > 0 {
                            row += &nothing.to_string();
                            nothing = 0;
                        }
                        row += match (piece, color) {
                            (Piece::NONE, Color::WHITE) => panic!("not possible"),
                            (Piece::NONE, Color::BLACK) => panic!("not possible"),
                            (Piece::PAWN, Color::WHITE) => "P",
                            (Piece::PAWN, Color::BLACK) => "p",
                            (Piece::KNIGHT, Color::WHITE) => "N",
                            (Piece::KNIGHT, Color::BLACK) => "n",
                            (Piece::BISHOP, Color::WHITE) => "B",
                            (Piece::BISHOP, Color::BLACK) => "b",
                            (Piece::ROOK, Color::WHITE) => "R",
                            (Piece::ROOK, Color::BLACK) => "r",
                            (Piece::QUEEN, Color::WHITE) => "Q",
                            (Piece::QUEEN, Color::BLACK) => "q",
                            (Piece::KING, Color::WHITE) => "K",
                            (Piece::KING, Color::BLACK) => "k",
                        }
                    }
                }
            }

            if nothing > 0 {
                row += &nothing.to_string();
            }

            rows.push(row);
        }

        let pos = rows.join("/");
        let turn = match self.turn {
            Color::WHITE => "w",
            Color::BLACK => "b",
        };

        let mut castling = String::new();
        if !self.castle_white_short
            && !self.castle_white_long
            && !self.castle_black_short
            && !self.castle_black_long
        {
            castling = "-".to_string();
        } else {
            if self.castle_white_short {
                castling += "K";
            }
            if self.castle_white_long {
                castling += "Q";
            }
            if self.castle_black_short {
                castling += "k";
            }
            if self.castle_black_long {
                castling += "q";
            }
        }

        let ep = if self.last_pos_ep == 0 {
            "-".to_string()
        } else {
            let pos = self.last_pos_ep.trailing_zeros();
            let x = pos % 8;
            let y = pos / 8;

            match x {
                7 => "a",
                6 => "b",
                5 => "c",
                4 => "d",
                3 => "e",
                2 => "f",
                1 => "g",
                0 => "h",
                _ => panic!("internal error"),
            }
            .to_string()
                + match y {
                    3 => "3",
                    4 => "6",
                    _ => panic!("invalid internal ep square"),
                }
        };

        return pos + " " + turn + " " + &castling + " " + &ep + " 0 0";
    }

    fn _white_occupied(&self) -> u64 {
        self.white_pawns
            | self.white_knights
            | self.white_bishops
            | self.white_rooks
            | self.white_queens
            | self.white_kings
    }

    fn _black_occupied(&self) -> u64 {
        self.black_pawns
            | self.black_knights
            | self.black_bishops
            | self.black_rooks
            | self.black_queens
            | self.black_kings
    }

    fn _white_knight_moves(&self) -> Vec<Move> {
        let mut knight_squares = self.white_knights;
        let mut moves: Vec<Move> = Vec::with_capacity(2);
        while knight_squares != 0 {
            let square: u64 = 1 << knight_squares.trailing_zeros();
            knight_squares = knight_squares & !square;
            moves.push(Move {
                from: square,
                to: KNIGHT_MOVES[square.trailing_zeros() as usize] & !self.white_occupied,
                promote_move: false,
                castle_move: CastleOpt::NONE,
                ep_move: false,
            });
        }
        return moves;
    }

    fn _black_knight_moves(&self) -> Vec<Move> {
        let mut knight_squares = self.black_knights;
        let mut moves: Vec<Move> = Vec::with_capacity(2);
        while knight_squares != 0 {
            let square: u64 = 1 << knight_squares.trailing_zeros();
            knight_squares = knight_squares & !square;
            moves.push(Move {
                from: square,
                to: KNIGHT_MOVES[square.trailing_zeros() as usize] & !self.black_occupied,
                promote_move: false,
                castle_move: CastleOpt::NONE,
                ep_move: false,
            });
        }
        return moves;
    }

    fn _white_king_moves(&self) -> Vec<Move> {
        assert_eq!(self.white_kings.count_ones(), 1);
        let to_squares: u64 = KING_MOVES[self.white_kings.trailing_zeros() as usize];

        let to = to_squares & !self.white_occupied;

        return vec![Move {
            from: self.white_kings,
            to: to,
            promote_move: false,
            castle_move: CastleOpt::NONE,
            ep_move: false,
        }];
    }

    fn _black_king_moves(&self) -> Vec<Move> {
        assert_eq!(self.black_kings.count_ones(), 1);
        let to_squares: u64 = KING_MOVES[self.black_kings.trailing_zeros() as usize];

        let to = to_squares & !self.black_occupied;

        return vec![Move {
            from: self.black_kings,
            to: to,
            promote_move: false,
            castle_move: CastleOpt::NONE,
            ep_move: false,
        }];
    }

    fn _white_in_dir(&self, change: &Dir, square: u64) -> u64 {
        let all_occupied = self.black_occupied | self.white_occupied;

        let path: u64 = get_ray(&change)[square.trailing_zeros() as usize];
        let occupied = path & all_occupied;
        let nearest = if occupied == 0 {
            0
        } else {
            match change {
                Dir::S | Dir::SE | Dir::SW | Dir::E => 63 - occupied.leading_zeros(),
                Dir::N | Dir::NE | Dir::NW | Dir::W => occupied.trailing_zeros(),
            }
        };
        return if nearest >= 64 {
            path & !self.white_occupied
        } else {
            get_short_ray(&change)[square.trailing_zeros() as usize][nearest as usize]
                & !self.white_occupied
        };
    }

    fn _black_in_dir(&self, change: &Dir, square: u64) -> u64 {
        let all_occupied = self.black_occupied | self.white_occupied;

        let path: u64 = get_ray(&change)[square.trailing_zeros() as usize];
        let occupied = path & all_occupied;
        let nearest = if occupied == 0 {
            0
        } else {
            match change {
                Dir::S | Dir::SE | Dir::SW | Dir::E => 63 - occupied.leading_zeros(),
                Dir::N | Dir::NE | Dir::NW | Dir::W => occupied.trailing_zeros(),
            }
        };
        return if nearest >= 64 {
            path & !self.black_occupied
        } else {
            get_short_ray(&change)[square.trailing_zeros() as usize][nearest as usize]
                & !self.black_occupied
        };
    }

    fn _white_rook_moves(&self) -> Vec<Move> {
        let mut rook_squares = self.white_rooks;
        let mut moves: Vec<Move> = Vec::with_capacity(2);
        while rook_squares != 0 {
            let square: u64 = 1 << rook_squares.trailing_zeros();
            rook_squares = rook_squares & !square;

            moves.push(Move {
                from: square,
                to: self._white_in_dir(&Dir::E, square)
                    | self._white_in_dir(&Dir::W, square)
                    | self._white_in_dir(&Dir::S, square)
                    | self._white_in_dir(&Dir::N, square),
                promote_move: false,
                castle_move: CastleOpt::NONE,
                ep_move: false,
            });
        }
        return moves;
    }

    fn _black_rook_moves(&self) -> Vec<Move> {
        let mut rook_squares = self.black_rooks;
        let mut moves: Vec<Move> = Vec::with_capacity(2);
        while rook_squares != 0 {
            let square: u64 = 1 << rook_squares.trailing_zeros();
            rook_squares = rook_squares & !square;

            moves.push(Move {
                from: square,
                to: self._black_in_dir(&Dir::E, square)
                    | self._black_in_dir(&Dir::W, square)
                    | self._black_in_dir(&Dir::S, square)
                    | self._black_in_dir(&Dir::N, square),
                promote_move: false,
                castle_move: CastleOpt::NONE,
                ep_move: false,
            });
        }
        return moves;
    }

    fn _white_bishop_moves(&self) -> Vec<Move> {
        let mut bishop_squares = self.white_bishops;
        let mut moves: Vec<Move> = Vec::with_capacity(2);
        while bishop_squares != 0 {
            let square: u64 = 1 << bishop_squares.trailing_zeros();
            bishop_squares = bishop_squares & !square;

            moves.push(Move {
                from: square,
                to: self._white_in_dir(&Dir::NE, square)
                    | self._white_in_dir(&Dir::NW, square)
                    | self._white_in_dir(&Dir::SE, square)
                    | self._white_in_dir(&Dir::SW, square),
                promote_move: false,
                castle_move: CastleOpt::NONE,
                ep_move: false,
            });
        }
        return moves;
    }

    fn _black_bishop_moves(&self) -> Vec<Move> {
        let mut bishop_squares = self.black_bishops;
        let mut moves: Vec<Move> = Vec::with_capacity(2);
        while bishop_squares != 0 {
            let square: u64 = 1 << bishop_squares.trailing_zeros();
            bishop_squares = bishop_squares & !square;

            moves.push(Move {
                from: square,
                to: self._black_in_dir(&Dir::NE, square)
                    | self._black_in_dir(&Dir::NW, square)
                    | self._black_in_dir(&Dir::SE, square)
                    | self._black_in_dir(&Dir::SW, square),
                promote_move: false,
                castle_move: CastleOpt::NONE,
                ep_move: false,
            });
        }
        return moves;
    }

    fn _white_queen_moves(&self) -> Vec<Move> {
        let mut queen_squares = self.white_queens;
        let mut moves: Vec<Move> = Vec::with_capacity(1);
        while queen_squares != 0 {
            let square: u64 = 1 << queen_squares.trailing_zeros();
            queen_squares = queen_squares & !square;

            moves.push(Move {
                from: square,
                to: self._white_in_dir(&Dir::NE, square)
                    | self._white_in_dir(&Dir::NW, square)
                    | self._white_in_dir(&Dir::SE, square)
                    | self._white_in_dir(&Dir::SW, square)
                    | self._white_in_dir(&Dir::N, square)
                    | self._white_in_dir(&Dir::S, square)
                    | self._white_in_dir(&Dir::E, square)
                    | self._white_in_dir(&Dir::W, square),
                promote_move: false,
                castle_move: CastleOpt::NONE,
                ep_move: false,
            });
        }
        return moves;
    }

    fn _black_queen_moves(&self) -> Vec<Move> {
        let mut queen_squares = self.black_queens;
        let mut moves: Vec<Move> = Vec::with_capacity(1);
        while queen_squares != 0 {
            let square: u64 = 1 << queen_squares.trailing_zeros();
            queen_squares = queen_squares & !square;

            moves.push(Move {
                from: square,
                to: self._black_in_dir(&Dir::NE, square)
                    | self._black_in_dir(&Dir::NW, square)
                    | self._black_in_dir(&Dir::SE, square)
                    | self._black_in_dir(&Dir::SW, square)
                    | self._black_in_dir(&Dir::N, square)
                    | self._black_in_dir(&Dir::S, square)
                    | self._black_in_dir(&Dir::E, square)
                    | self._black_in_dir(&Dir::W, square),
                promote_move: false,
                castle_move: CastleOpt::NONE,
                ep_move: false,
            });
        }
        return moves;
    }

    fn _white_pawn_moves(&self) -> Vec<Move> {
        let mut pawn_squares = self.white_pawns;
        let mut moves: Vec<Move> = Vec::with_capacity(8);
        while pawn_squares != 0 {
            let square: u64 = 1 << pawn_squares.trailing_zeros();
            pawn_squares = pawn_squares & !square;
            let mut move_squares: u64 = 0;
            let up1 = move_n(square);
            if up1 & self.white_occupied == 0 && up1 & self.black_occupied == 0 {
                move_squares |= up1;
                if square & RANK_2 != 0 {
                    let up2 = move_n(up1);
                    if up2 & self.white_occupied == 0 && up2 & self.black_occupied == 0 {
                        move_squares |= up2;
                    }
                }
            }

            if move_ne(square) & self.black_occupied != 0 {
                move_squares |= move_ne(square);
            }
            if move_nw(square) & self.black_occupied != 0 {
                move_squares |= move_nw(square);
            }

            moves.push(Move {
                from: square,
                to: move_squares,
                promote_move: if square & RANK_7 != 0 { true } else { false },
                castle_move: CastleOpt::NONE,
                ep_move: false,
            });
        }
        return moves;
    }

    fn _black_pawn_moves(&self) -> Vec<Move> {
        let mut pawn_squares = self.black_pawns;
        let mut moves: Vec<Move> = Vec::with_capacity(8);
        while pawn_squares != 0 {
            let square: u64 = 1 << pawn_squares.trailing_zeros();
            pawn_squares = pawn_squares & !square;
            let mut move_squares: u64 = 0;
            let up1 = move_s(square);
            if up1 & self.white_occupied == 0 && up1 & self.black_occupied == 0 {
                move_squares |= up1;
                if square & RANK_7 != 0 {
                    let up2 = move_s(up1);
                    if up2 & self.white_occupied == 0 && up2 & self.black_occupied == 0 {
                        move_squares |= up2;
                    }
                }
            }

            if move_se(square) & self.white_occupied != 0 {
                move_squares |= move_se(square);
            }
            if move_sw(square) & self.white_occupied != 0 {
                move_squares |= move_sw(square);
            }

            moves.push(Move {
                from: square,
                to: move_squares,
                promote_move: if square & RANK_2 != 0 { true } else { false },
                castle_move: CastleOpt::NONE,
                ep_move: false,
            });
        }
        return moves;
    }

    fn _white_castle_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        if self.castle_white_short
            && self.black_attacks() & CASTLE_WS_PATH == 0
            && self.white_occupied & CASTLE_WS_EMPTY == 0
            && self.black_occupied & CASTLE_WS_EMPTY == 0
        {
            moves.push(Move {
                from: E1,
                to: G1,
                promote_move: false,
                castle_move: CastleOpt::WHITESHORT,
                ep_move: false,
            });
        }

        if self.castle_white_long
            && self.black_attacks() & CASTLE_WL_PATH == 0
            && self.white_occupied & CASTLE_WL_EMPTY == 0
            && self.black_occupied & CASTLE_WL_EMPTY == 0
        {
            moves.push(Move {
                from: E1,
                to: C1,
                promote_move: false,
                castle_move: CastleOpt::WHITELONG,
                ep_move: false,
            });
        }

        return moves;
    }

    fn _black_castle_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        if self.castle_black_short
            && self.white_attacks() & CASTLE_BS_PATH == 0
            && self.black_occupied & CASTLE_BS_EMPTY == 0
            && self.white_occupied & CASTLE_BS_EMPTY == 0
        {
            moves.push(Move {
                from: E8,
                to: G8,
                promote_move: false,
                castle_move: CastleOpt::BLACKSHORT,
                ep_move: false,
            });
        }

        if self.castle_black_long
            && self.white_attacks() & CASTLE_BL_PATH == 0
            && self.black_occupied & CASTLE_BL_EMPTY == 0
            && self.white_occupied & CASTLE_BL_EMPTY == 0
        {
            moves.push(Move {
                from: E8,
                to: C8,
                promote_move: false,
                castle_move: CastleOpt::BLACKLONG,
                ep_move: false,
            });
        }

        return moves;
    }

    fn _white_ep_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        if self.last_pos_ep == 0 {
            return vec![];
        }

        let left = move_w(self.last_pos_ep) & self.white_pawns;
        if left != 0 {
            moves.push(Move {
                from: left,
                to: move_n(self.last_pos_ep),
                promote_move: false,
                castle_move: CastleOpt::NONE,
                ep_move: true,
            });
        }

        let right = move_e(self.last_pos_ep) & self.white_pawns;
        if right != 0 {
            moves.push(Move {
                from: right,
                to: move_n(self.last_pos_ep),
                promote_move: false,
                castle_move: CastleOpt::NONE,
                ep_move: true,
            });
        }

        return moves;
    }

    fn _black_ep_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        if self.last_pos_ep == 0 {
            return vec![];
        }

        let left = move_w(self.last_pos_ep) & self.black_pawns;
        if left != 0 {
            moves.push(Move {
                from: left,
                to: move_s(self.last_pos_ep),
                promote_move: false,
                castle_move: CastleOpt::NONE,
                ep_move: true,
            });
        }

        let right = move_e(self.last_pos_ep) & self.black_pawns;
        if right != 0 {
            moves.push(Move {
                from: right,
                to: move_s(self.last_pos_ep),
                promote_move: false,
                castle_move: CastleOpt::NONE,
                ep_move: true,
            });
        }

        return moves;
    }

    fn _all_white_moves(&self, include_castling: bool) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        moves.extend(self._white_knight_moves());
        moves.extend(self._white_bishop_moves());
        moves.extend(self._white_rook_moves());
        moves.extend(self._white_queen_moves());
        moves.extend(self._white_king_moves());
        if include_castling {
            moves.extend(self._white_pawn_moves());
            moves.extend(self._white_castle_moves());
        }
        moves.extend(self._white_ep_moves());
        return moves;
    }

    fn _all_black_moves(&self, include_castling: bool) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        moves.extend(self._black_knight_moves());
        moves.extend(self._black_bishop_moves());
        moves.extend(self._black_rook_moves());
        moves.extend(self._black_queen_moves());
        moves.extend(self._black_king_moves());
        if include_castling {
            moves.extend(self._black_pawn_moves());
            moves.extend(self._black_castle_moves());
        }
        moves.extend(self._black_ep_moves());
        return moves;
    }

    pub fn all_white_moves(&self) -> Vec<SMove> {
        if self.outcome != Outcome::ONGOING {
            return Vec::new();
        }
        let moves = self._all_white_moves(true);
        let mut smoves: Vec<SMove> = Vec::new();

        for m in moves {
            for sm in m.get_moves() {
                smoves.push(sm);
            }
        }

        return smoves;
    }

    pub fn all_black_moves(&self) -> Vec<SMove> {
        if self.outcome != Outcome::ONGOING {
            return Vec::new();
        }
        let moves = self._all_black_moves(true);
        let mut smoves: Vec<SMove> = Vec::new();

        for m in moves {
            for sm in m.get_moves() {
                smoves.push(sm);
            }
        }

        return smoves;
    }

    fn set_none(&mut self, pos: u8) {
        let square: u64 = 1 << pos;

        self.white_pawns &= !square;
        self.white_knights &= !square;
        self.white_bishops &= !square;
        self.white_rooks &= !square;
        self.white_queens &= !square;
        self.white_kings &= !square;

        self.black_pawns &= !square;
        self.black_knights &= !square;
        self.black_bishops &= !square;
        self.black_rooks &= !square;
        self.black_queens &= !square;
        self.black_kings &= !square;
    }

    fn set_piece(&mut self, pos: u8, piece: Piece, color: Color) {
        let square: u64 = 1 << pos;

        let bitboards: [&mut u64; 6] = match color {
            Color::WHITE => [
                &mut self.white_pawns,
                &mut self.white_knights,
                &mut self.white_bishops,
                &mut self.white_rooks,
                &mut self.white_queens,
                &mut self.white_kings,
            ],
            Color::BLACK => [
                &mut self.black_pawns,
                &mut self.black_knights,
                &mut self.black_bishops,
                &mut self.black_rooks,
                &mut self.black_queens,
                &mut self.black_kings,
            ],
        };

        match piece {
            Piece::NONE => self.set_none(pos),
            Piece::ROOK => *bitboards[3] |= square,
            Piece::KING => *bitboards[5] |= square,
            Piece::PAWN => *bitboards[0] |= square,
            Piece::QUEEN => *bitboards[4] |= square,
            Piece::BISHOP => *bitboards[2] |= square,
            Piece::KNIGHT => *bitboards[1] |= square,
        }
    }

    fn get_piece_at(&self, pos: u8) -> (Piece, Color) {
        let square: u64 = 1 << pos;

        if square & self.white_pawns != 0 {
            return (Piece::PAWN, Color::WHITE);
        }
        if square & self.white_knights != 0 {
            return (Piece::KNIGHT, Color::WHITE);
        }
        if square & self.white_bishops != 0 {
            return (Piece::BISHOP, Color::WHITE);
        }
        if square & self.white_rooks != 0 {
            return (Piece::ROOK, Color::WHITE);
        }
        if square & self.white_queens != 0 {
            return (Piece::QUEEN, Color::WHITE);
        }
        if square & self.white_kings != 0 {
            return (Piece::KING, Color::WHITE);
        }
        if square & self.black_pawns != 0 {
            return (Piece::PAWN, Color::BLACK);
        }
        if square & self.black_knights != 0 {
            return (Piece::KNIGHT, Color::BLACK);
        }
        if square & self.black_bishops != 0 {
            return (Piece::BISHOP, Color::BLACK);
        }
        if square & self.black_rooks != 0 {
            return (Piece::ROOK, Color::BLACK);
        }
        if square & self.black_queens != 0 {
            return (Piece::QUEEN, Color::BLACK);
        }
        if square & self.black_kings != 0 {
            return (Piece::KING, Color::BLACK);
        }

        return (Piece::NONE, Color::WHITE);
    }

    fn set_new_piece(&mut self, pos: u8, piece: Piece, color: Color) {
        self.set_none(pos);
        self.set_piece(pos, piece, color);
    }

    pub fn white_attacks(&self) -> u64 {
        let moves = self._all_white_moves(false);
        let mut attacked: u64 = 0;
        for m in moves {
            attacked |= m.to;
        }

        attacked |= move_nw(self.white_pawns) | move_ne(self.white_pawns);
        return attacked;
    }

    pub fn black_attacks(&self) -> u64 {
        let moves = self._all_black_moves(false);
        let mut attacked: u64 = 0;
        for m in moves {
            attacked |= m.to;
        }

        attacked |= move_sw(self.black_pawns) | move_se(self.black_pawns);
        return attacked;
    }

    fn is_check(&self, color: Color) -> bool {
        let king_pos = if color == Color::WHITE {
            self.white_kings
        } else {
            self.black_kings
        };
        let attacked = match color {
            Color::WHITE => self.black_attacks(),
            Color::BLACK => self.white_attacks(),
        };

        return king_pos & attacked != 0;
    }

    fn is_legal_pos(&self) -> bool {
        match self.turn {
            Color::WHITE => !self.is_check(Color::BLACK),
            Color::BLACK => !self.is_check(Color::WHITE),
        }
    }

    fn make_castle_move(&mut self, smove: SMove) {
        match smove.castle_move {
            CastleOpt::NONE => panic!("castling move expected"),
            CastleOpt::WHITESHORT => {
                self.white_rooks ^= H1;
                self.white_kings = G1;
                self.white_rooks |= F1;

                self.castle_white_short = false;
                self.castle_white_long = false;
            }
            CastleOpt::BLACKSHORT => {
                self.black_rooks ^= H8;
                self.black_kings = G8;
                self.black_rooks |= F8;

                self.castle_black_short = false;
                self.castle_black_long = false;
            }
            CastleOpt::WHITELONG => {
                self.white_rooks ^= A1;
                self.white_kings = C1;
                self.white_rooks |= D1;

                self.castle_white_short = false;
                self.castle_white_long = false;
            }
            CastleOpt::BLACKLONG => {
                self.black_rooks ^= A8;
                self.black_kings = C8;
                self.black_rooks |= D8;

                self.castle_black_short = false;
                self.castle_black_long = false;
            }
        }
    }

    #[wasm_bindgen]
    pub fn make_move(&mut self, smove: SMove) {
        let before = *self;
        self.last_pos_ep = 0;
        if smove.castle_move != CastleOpt::NONE {
            self.make_castle_move(smove);
        } else {
            if smove.promote_piece == Piece::NONE {
                let (piece, color) = self.get_piece_at(smove.from);
                let square: u64 = 1 << smove.from;
                match (piece, color) {
                    (Piece::NONE, Color::WHITE) => {}
                    (Piece::NONE, Color::BLACK) => {}
                    (Piece::PAWN, Color::WHITE) => self.white_pawns ^= square,
                    (Piece::PAWN, Color::BLACK) => self.black_pawns ^= square,
                    (Piece::KNIGHT, Color::WHITE) => self.white_knights ^= square,
                    (Piece::KNIGHT, Color::BLACK) => self.black_knights ^= square,
                    (Piece::BISHOP, Color::WHITE) => self.white_bishops ^= square,
                    (Piece::BISHOP, Color::BLACK) => self.black_bishops ^= square,
                    (Piece::ROOK, Color::WHITE) => self.white_rooks ^= square,
                    (Piece::ROOK, Color::BLACK) => self.black_rooks ^= square,
                    (Piece::QUEEN, Color::WHITE) => self.white_queens ^= square,
                    (Piece::QUEEN, Color::BLACK) => self.black_queens ^= square,
                    (Piece::KING, Color::WHITE) => self.white_kings ^= square,
                    (Piece::KING, Color::BLACK) => self.black_kings ^= square,
                };
                self.set_new_piece(smove.to, piece, color);
                if smove.ep_move {
                    self.set_none(match color {
                        Color::WHITE => smove.to - 8,
                        Color::BLACK => smove.to + 8,
                    });
                }

                if piece == Piece::KING {
                    match color {
                        Color::WHITE => {
                            self.castle_white_short = false;
                            self.castle_white_long = false;
                        }
                        Color::BLACK => {
                            self.castle_black_short = false;
                            self.castle_black_long = false;
                        }
                    }
                } else if piece == Piece::ROOK {
                    match color {
                        Color::WHITE => {
                            if smove.from == 0 {
                                self.castle_white_short = false;
                            } else if smove.from == 7 {
                                self.castle_white_long = false;
                            }
                        }
                        Color::BLACK => {
                            if smove.from == 56 {
                                self.castle_black_short = false;
                            } else if smove.from == 63 {
                                self.castle_black_long = false;
                            }
                        }
                    }
                } else if piece == Piece::PAWN {
                    self.reset_fifty_move_rule();
                    let to_square: u64 = 1 << smove.to;
                    let from_square: u64 = 1 << smove.from;
                    if (color == Color::WHITE
                        && to_square & RANK_4 != 0
                        && from_square & RANK_2 != 0)
                        || (color == Color::BLACK
                            && to_square & RANK_5 != 0
                            && from_square & RANK_7 != 0)
                    {
                        self.last_pos_ep = to_square;
                    }
                }
            } else {
                self.reset_fifty_move_rule();
                self.set_none(smove.from);
                self.set_new_piece(smove.to, smove.promote_piece, self.turn);
            }

            let to: u64 = 1 << smove.to;
            if to == A1 {
                self.castle_white_long = false;
            }
            if to == A8 {
                self.castle_black_long = false;
            }
            if to == H1 {
                self.castle_white_short = false;
            }
            if to == H8 {
                self.castle_black_short = false;
            }

            match self.get_piece_at(smove.to) {
                (Piece::NONE, Color::WHITE) => self.reset_fifty_move_rule(),
                (Piece::NONE, Color::BLACK) => self.reset_fifty_move_rule(),
                _ => {}
            }
        }
        self.turn = match self.turn {
            Color::WHITE => Color::BLACK,
            Color::BLACK => Color::WHITE,
        };

        if self.white_kings.count_ones() != 1 || self.black_kings.count_ones() != 1 {
            println!("a king dissappeared:");
            before.print();
            print!("\n");
            smove.print();
            print!("\n");
            self.print();
            panic!()
        }

        self.black_occupied = self._black_occupied();
        self.white_occupied = self._white_occupied();
    }

    pub fn play(&mut self, smove: SMove) {
        self.make_move(smove);
        if self.next_index < 100 {
            self.previous_positions[self.next_index] = self.get_pos();
            self.next_index += 1;
        }

        if self.get_moves().len() == 0 {
            self.outcome = if self.is_check(self.turn) {
                match self.turn {
                    Color::WHITE => Outcome::BLACK,
                    Color::BLACK => Outcome::WHITE,
                }
            } else {
                Outcome::DRAW
            }
        }
        if self.is_threefold_rep() {
            self.outcome = Outcome::DRAW;
        }

        if self.next_index >= 100 {
            self.outcome = Outcome::DRAW;
            self.next_index = 0;
        }
    }

    fn reset_fifty_move_rule(&mut self) {
        self.next_index = 0;
        self.previous_positions = [[0; 12]; 100];
    }

    pub fn print(&self) {
        for row in self.get_rows() {
            println!("{}", row);
        }
    }

    #[wasm_bindgen]
    pub fn get_rows(&self) -> Vec<String> {
        let mut rows = Vec::new();
        for y in (0..8).rev() {
            let mut row = String::new();
            for x in (0..8).rev() {
                let (piece, color) = self.get_piece_at(y * 8 + x);
                let mut next = match piece {
                    Piece::NONE => " ",
                    Piece::PAWN => "P",
                    Piece::KNIGHT => "N",
                    Piece::BISHOP => "B",
                    Piece::ROOK => "R",
                    Piece::QUEEN => "Q",
                    Piece::KING => "K",
                }
                .to_string();

                if color == Color::BLACK {
                    next = next.to_lowercase();
                }
                row += &next;
            }
            rows.push(row);
        }

        return rows;
    }

    pub fn get_next_boards(&self) -> Vec<Board> {
        let mut boards: Vec<Board> = Vec::new();
        let moves = match self.turn {
            Color::WHITE => self.all_white_moves(),
            Color::BLACK => self.all_black_moves(),
        };

        for smove in moves {
            let mut new = *self;
            new.make_move(smove);
            if new.is_legal_pos() {
                boards.push(new);
            }
        }

        return boards;
    }

    pub fn get_next_move_boards(&self) -> Vec<MoveBoardPair> {
        let mut boards: Vec<MoveBoardPair> = Vec::new();
        let moves = match self.turn {
            Color::WHITE => self.all_white_moves(),
            Color::BLACK => self.all_black_moves(),
        };

        for smove in moves {
            let mut new = *self;
            new.make_move(smove);
            if new.is_legal_pos() {
                boards.push(MoveBoardPair { 0: smove, 1: new });
            }
        }

        return boards;
    }

    #[wasm_bindgen]
    pub fn get_moves(&self) -> Vec<SMove> {
        let all = match self.turn {
            Color::WHITE => self.all_white_moves(),
            Color::BLACK => self.all_black_moves(),
        };
        let mut legal: Vec<SMove> = Vec::new();
        for smove in all {
            let mut new = self.clone();
            new.make_move(smove);
            if new.is_legal_pos() {
                legal.push(smove);
            }
        }

        return legal;
    }

    #[wasm_bindgen]
    pub fn get_moves_square(&self, square: u8) -> Vec<SMove> {
        self.get_moves()
            .into_iter()
            .filter(|smove| smove.from == square)
            .collect()
    }

    pub fn is_checkmate(&self) -> bool {
        self.is_check(self.turn) && self.get_moves().len() == 0
    }

    pub fn is_stalemate(&self) -> bool {
        !(self.is_check(self.turn)) && self.get_moves().len() == 0
    }

    fn get_pos(&self) -> [u64; 12] {
        [
            self.white_pawns,
            self.white_knights,
            self.white_bishops,
            self.white_rooks,
            self.white_queens,
            self.white_kings,
            self.black_pawns,
            self.black_knights,
            self.black_bishops,
            self.black_rooks,
            self.black_queens,
            self.black_kings,
        ]
    }

    pub fn is_threefold_rep(&self) -> bool {
        let now = self.get_pos();
        let mut count = 0;
        for pos in self.previous_positions {
            if pos == now {
                count += 1;
                if count >= 3 {
                    return true;
                }
            }
        }
        return false;
    }
}

mod engine;
