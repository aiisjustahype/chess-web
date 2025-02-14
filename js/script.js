import init, { Board, Piece } from "./../pkg/rust.js";

await init();
let board = new Board();
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
  board.make_move(promote_move);
  popup.style.display = "none";
  make_board();
});

promote_bishop.addEventListener("click", () => {
  promote_move.promote_piece = Piece.BISHOP;
  board.make_move(promote_move);
  popup.style.display = "none";
  make_board();
});

promote_rook.addEventListener("click", () => {
  promote_move.promote_piece = Piece.ROOK;
  board.make_move(promote_move);
  popup.style.display = "none";
  make_board();
});

promote_queen.addEventListener("click", () => {
  promote_move.promote_piece = Piece.QUEEN;
  board.make_move(promote_move);
  popup.style.display = "none";
  make_board();
});

function get_style(name) {
  return [...document.styleSheets[0].cssRules].filter(m => m.selectorText == name)[0];
}

function set_board_size() {
  let width = window.innerWidth;
  let height = window.innerHeight;
  let l = (width > height - 60 ? height - 60 : width) - 50;
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
          "P": "./../assets/white-pawn.png",
          "p": "./../assets/black-pawn.png",
          "N": "./../assets/white-knight.png",
          "n": "./../assets/black-knight.png",
          "B": "./../assets/white-bishop.png",
          "b": "./../assets/black-bishop.png",
          "R": "./../assets/white-rook.png",
          "r": "./../assets/black-rook.png",
          "Q": "./../assets/white-queen.png",
          "q": "./../assets/black-queen.png",
          "K": "./../assets/white-king.png",
          "k": "./../assets/black-king.png",
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

function square_click(x, y) {
  let square = 63 - (y * 8 + x);
  if (moves.map(m => m.to).includes(square)) {
    for (let i = 0; i < moves.length; i++) {
      if (moves[i].to == square) {
        if (moves[i].promote_piece == Piece.NONE) {
          board.make_move(moves[i]);
          break;
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

make_board();
