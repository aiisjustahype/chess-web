import init, { Board, Piece } from "./pkg/rust.js";

await init();
let board = new Board();
let boards = [board.get_clone()];
let board_i = 0;

let moves = [];
let promote_move = null;
let popup = document.getElementById("promote-popup");
let promote_knight = document.getElementById("promote-knight");
let promote_bishop = document.getElementById("promote-bishop");
let promote_rook = document.getElementById("promote-rook");
let promote_queen = document.getElementById("promote-queen");

window.addEventListener("click", (event) => {
  if (event.target === popup) {
    popup.style.display = "none";
  }
});

promote_knight.addEventListener("click", () => {
  promote_move.promote_piece = Piece.KNIGHT;
  play_move(promote_move);
  popup.style.display = "none";
  make_board();
});

promote_bishop.addEventListener("click", () => {
  promote_move.promote_piece = Piece.BISHOP;
  play_move(promote_move);
  popup.style.display = "none";
  make_board();
});

promote_rook.addEventListener("click", () => {
  promote_move.promote_piece = Piece.ROOK;
  play_move(promote_move);
  popup.style.display = "none";
  make_board();
});

promote_queen.addEventListener("click", () => {
  promote_move.promote_piece = Piece.QUEEN;
  play_move(promote_move);
  popup.style.display = "none";
  make_board();
});

function get_style(name) {
  return [...document.styleSheets[0].cssRules].filter(m => m.selectorText == name)[0];
}

function set_board_size() {
  let width = window.innerWidth;
  let height = window.innerHeight;
  let l = (width > height - 100 ? height - 100 : width) - 50;
  let square_l = l / 8;
  let square_style = get_style(".square");
  square_style.style["width"] = square_l + "px";
  square_style.style["height"] = square_l + "px";
  square_style = get_style(".square-red");
  square_style.style["width"] = square_l + "px";
  square_style.style["height"] = square_l + "px";
  let board_style = get_style("#board");
  board_style.style["width"] = l + "px";
  board_style.style["height"] = l + "px";
  let piece_style = get_style(".piece");
  piece_style.style["width"] = square_l + "px";
  piece_style.style["height"] = square_l + "px";
}

set_board_size();
window.onresize = set_board_size;

function make_board() {
  let board_div = document.getElementById("board");
  board_div.innerHTML = "";
  let rows = board.get_rows();
  for (let row = 0; row < 8; row++) {
    for (let square = 0; square < 8; square++) {
      let s = document.createElement("div");
      s.id = "square" + row + square;
      if ((row * 7 + square) % 2 != 0) {
        s.className = "square-red";
      } else {
        s.className = "square";
      }
      if (row == 0 && square == 0) {
        s.classList.add("topleft");
      }
      else if (row == 0 && square == 7) {
        s.classList.add("topright");
      }
      else if (row == 7 && square == 0) {
        s.classList.add("bottomleft");
      }
      else if (row == 7 && square == 7) {
        s.classList.add("bottomright");
      }
      let index = 63 - (row * 8 + square);
      if (moves.map(m => m.to == index).includes(true)) {
        s.classList.add("target-square");
      }
      let piece = rows[row][square];
      if (piece != " ") {
        let pic = {
          "P": "./assets/white-pawn.png",
          "p": "./assets/black-pawn.png",
          "N": "./assets/white-knight.png",
          "n": "./assets/black-knight.png",
          "B": "./assets/white-bishop.png",
          "b": "./assets/black-bishop.png",
          "R": "./assets/white-rook.png",
          "r": "./assets/black-rook.png",
          "Q": "./assets/white-queen.png",
          "q": "./assets/black-queen.png",
          "K": "./assets/white-king.png",
          "k": "./assets/black-king.png",
        }[piece];
        let e = document.createElement("img");
        e.src = pic;
        e.className = "piece";
        s.appendChild(e);
      }
      s.addEventListener("click", () => { square_click(square, row) });
      board_div.appendChild(s);
    }
  }
}

function play_move(m) {
  if (board_i != boards.length - 1) {
    boards = boards.slice(0, board_i + 1);
  }
  board.make_move(m);
  boards.push(board.get_clone());
  board_i += 1;
}

function previous_board() {
  if (board_i > 0) {
    board_i--;
    board = boards[board_i].get_clone();
  }
  moves = [];
  make_board();
}

function next_board() {
  if (board_i + 1 < boards.length) {
    board_i++;
    board = boards[board_i].get_clone();
  }
  moves = [];
  make_board();
}

function first_board() {
  board_i = 0;
  board = boards[board_i].get_clone();
  moves = [];
  make_board();
}

function last_board() {
  board_i = boards.length - 1;
  board = boards[board_i].get_clone();
  moves = [];
  make_board();
}

let b_first = document.getElementById("b_first");
let b_previous = document.getElementById("b_previous");
let b_next = document.getElementById("b_next");
let b_last = document.getElementById("b_last");

b_first.addEventListener("click", first_board);
b_previous.addEventListener("click", previous_board);
b_next.addEventListener("click", next_board);
b_last.addEventListener("click", last_board);

function square_click(x, y) {
  let square = 63 - (y * 8 + x);
  if (moves.map(m => m.to).includes(square)) {
    for (let i = 0; i < moves.length; i++) {
      if (moves[i].to == square) {
        if (moves[i].promote_piece == Piece.NONE) {
          play_move(moves[i]);
        } else {
          popup.style.display = "block";
          promote_move = moves[i];
        }
      }
    }
    moves = [];
  } else {
    moves = board.get_moves_square(square);
  }
  make_board();
}

let headline = document.getElementById("opennav");
let sidenav = document.getElementById("sidenav");
let closebtn = document.getElementById("close-btn");
let fog = document.getElementById("fog");
headline.addEventListener("click", () => {
  sidenav.style.width = "min(40%, 500px)";
  fog.style.display = "block";
});
closebtn.addEventListener("click", () => {
  sidenav.style.width = "0";
  fog.style.display = "none";
})

make_board();
