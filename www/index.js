import { Game, Board, Painter, Team, init_game } from "draught";
import { memory } from "draught/draught_bg.wasm";

init_game();

const CANVAS_WIDTH = 480;
const CANVAS_HEIGHT = 480;

const BOARD_WIDTH = 8;
const BOARD_HEIGHT = 8;

const PIECE_ROWS = 3;

const canvas = document.getElementById("game-canvas");
canvas.addEventListener("click", (event) => {
    var mousepos = getMousePos(canvas, event);
    // console.log(mousepos);
    var cell = {
        x: Math.floor((mousepos.x / CANVAS_WIDTH) * BOARD_WIDTH),
        y: Math.floor((mousepos.y / CANVAS_HEIGHT) * BOARD_HEIGHT),
    }
    console.log(cell);
})

function getMousePos(canvas, evt) {
    var rect = canvas.getBoundingClientRect();
    return {
        x: evt.clientX - rect.left,
        y: evt.clientY - rect.top
    };
}

let painter = new Painter(CANVAS_WIDTH, CANVAS_HEIGHT, "game-canvas");

// let board = new Board(BOARD_WIDTH, BOARD_HEIGHT, Team.Black);
// painter.draw(board);

let game = new Game(BOARD_WIDTH, BOARD_HEIGHT, PIECE_ROWS, Team.Black, "game-canvas", CANVAS_WIDTH, CANVAS_HEIGHT);
game.set_painter(painter);
game.draw();
