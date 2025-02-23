/* tslint:disable */
/* eslint-disable */
export function get_best_move(board: Board, depth: number): SMove;
export enum CastleOpt {
  NONE = 0,
  WHITESHORT = 1,
  BLACKSHORT = 2,
  WHITELONG = 3,
  BLACKLONG = 4,
}
export enum Color {
  WHITE = 0,
  BLACK = 1,
}
export enum Piece {
  NONE = 0,
  PAWN = 1,
  KNIGHT = 2,
  BISHOP = 3,
  ROOK = 4,
  QUEEN = 5,
  KING = 6,
}
export class Board {
  free(): void;
  constructor();
  get_clone(): Board;
  fen(): string;
  all_white_moves(): SMove[];
  all_black_moves(): SMove[];
  white_attacks(): bigint;
  black_attacks(): bigint;
  make_move(smove: SMove): void;
  print(): void;
  get_rows(): string[];
  get_next_boards(): Board[];
  get_next_move_boards(): MoveBoardPair[];
  get_moves(): SMove[];
  get_moves_square(square: number): SMove[];
  is_checkmate(): boolean;
  is_stalemate(): boolean;
  turn: Color;
}
export class MoveBoardPair {
  private constructor();
  free(): void;
  0: SMove;
  1: Board;
}
export class SMove {
  private constructor();
  free(): void;
  print(): void;
  from: number;
  to: number;
  promote_piece: Piece;
  castle_move: CastleOpt;
  ep_move: boolean;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_smove_free: (a: number, b: number) => void;
  readonly __wbg_get_smove_from: (a: number) => number;
  readonly __wbg_set_smove_from: (a: number, b: number) => void;
  readonly __wbg_get_smove_to: (a: number) => number;
  readonly __wbg_set_smove_to: (a: number, b: number) => void;
  readonly __wbg_get_smove_promote_piece: (a: number) => number;
  readonly __wbg_set_smove_promote_piece: (a: number, b: number) => void;
  readonly __wbg_get_smove_castle_move: (a: number) => number;
  readonly __wbg_set_smove_castle_move: (a: number, b: number) => void;
  readonly __wbg_get_smove_ep_move: (a: number) => number;
  readonly __wbg_set_smove_ep_move: (a: number, b: number) => void;
  readonly __wbg_board_free: (a: number, b: number) => void;
  readonly __wbg_get_board_turn: (a: number) => number;
  readonly __wbg_set_board_turn: (a: number, b: number) => void;
  readonly smove_print: (a: number) => void;
  readonly __wbg_moveboardpair_free: (a: number, b: number) => void;
  readonly __wbg_get_moveboardpair_0: (a: number) => number;
  readonly __wbg_set_moveboardpair_0: (a: number, b: number) => void;
  readonly __wbg_get_moveboardpair_1: (a: number) => number;
  readonly __wbg_set_moveboardpair_1: (a: number, b: number) => void;
  readonly board_new: () => number;
  readonly board_get_clone: (a: number) => number;
  readonly board_fen: (a: number) => [number, number];
  readonly board_all_white_moves: (a: number) => [number, number];
  readonly board_all_black_moves: (a: number) => [number, number];
  readonly board_white_attacks: (a: number) => bigint;
  readonly board_black_attacks: (a: number) => bigint;
  readonly board_make_move: (a: number, b: number) => void;
  readonly board_print: (a: number) => void;
  readonly board_get_rows: (a: number) => [number, number];
  readonly board_get_next_boards: (a: number) => [number, number];
  readonly board_get_next_move_boards: (a: number) => [number, number];
  readonly board_get_moves: (a: number) => [number, number];
  readonly board_get_moves_square: (a: number, b: number) => [number, number];
  readonly board_is_checkmate: (a: number) => number;
  readonly board_is_stalemate: (a: number) => number;
  readonly get_best_move: (a: number, b: number) => number;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
