import { Game, Board, BrdIdx, Painter, Team, init_wasm, Moveable, SquareState } from "draught";
import { memory } from "draught/draught_bg.wasm";

///////////////////
//    CONSTS
///////////////////

const CANVAS_WIDTH = 480;
const CANVAS_HEIGHT = 480;

const BOARD_WIDTH = 8;
const BOARD_HEIGHT = 8;

const PIECE_ROWS = 3;

const GameState = {
    HUMAN_TURN: {
        THINKING: "human_turn.thinking",
        FROM_SELECTED: "human_turn.from_selected"
    },
    AI_TURN: "ai_turn"
}

//////////////////
//  GAME STUFF
//////////////////

init_wasm();

// let board = new Board(BOARD_WIDTH, BOARD_HEIGHT, Team.Black);

const statusText = document.getElementById("status-p");

let current_state = GameState.HUMAN_TURN.THINKING;
let painter = new Painter(CANVAS_WIDTH, CANVAS_HEIGHT, "game-canvas");
// painter.draw(board);

let clicks = [];

let from = null;
let to = null;

let game = new Game(BOARD_WIDTH, BOARD_HEIGHT, PIECE_ROWS, Team.Black);
game.set_painter(painter);
game.draw();

/////////////////
//   CANVAS
/////////////////

const canvas = document.getElementById("game-canvas");
canvas.addEventListener("click", (event) => {
    var mousepos = getMousePos(canvas, event);
    // console.log(mousepos);
    var cell = new BrdIdx(
        Math.floor((mousepos.y / CANVAS_HEIGHT) * BOARD_HEIGHT),
        Math.floor((mousepos.x / CANVAS_WIDTH) * BOARD_WIDTH),
    );
    // console.log(cell);
    process_canvas_click(cell);
})

////////////////
//   FUNCS
////////////////

function start_game() {
    game = new Game(BOARD_WIDTH, BOARD_HEIGHT, PIECE_ROWS, Team.Black);
    game.set_painter(painter);
    game.draw();

    current_state = GameState.HUMAN_TURN.THINKING;
}

function process_canvas_click(cell_coord) {

    // if (game.current_cell_state(cell_coord).state == SquareState.Unplayable ) {
    //     from = null;
    //     to = null;
    //     current_state = GameState.HUMAN_TURN.THINKING;
    //     setStatusText("Unplayable Square!");
    //     return;
    // }

    switch(current_state) {
        case GameState.HUMAN_TURN.THINKING:
            console.log("Your turn, first piece picked");

            clicks.push(cell_coord);

            // from = cell_coord;
            current_state = GameState.HUMAN_TURN.FROM_SELECTED; 
            
            break;
        case GameState.HUMAN_TURN.FROM_SELECTED:
            console.log("Your turn, first piece already picked, picking second");

            clicks.push(cell_coord);

            // to = cell_coord;

            if (clicks.length != 2) {
                setStatusText(`Error: wrong number of clicks to process ${clicks.length}`);
                console.error(`Error: wrong number of clicks to process ${clicks.length}`);

                return;
            }

            // console.log(clicks[0].eq(clicks[1]));

            if (clicks[0].eq(clicks[1])) {
                setStatusText("Move Cancelled");
            } else {

                let status = game.make_move(clicks[0], clicks[1]);

                switch(status) {
                    case Moveable.Allowed:
                        break;
                    case Moveable.IllegalTrajectory:
                        break;
                    case Moveable.JumpingSameTeam:
                        break;
                    case Moveable.NoJumpablePiece:
                        break;
                    case Moveable.OccupiedDest:
                        break;
                    case Moveable.OutOfBounds:
                        break;
                    case Moveable.UnoccupiedSrc:
                        break;
                    case Moveable.Unplayable:
                        break;
                    case Moveable.WrongTeamSrc:
                        break;
                }
                
            }

            game.draw();
            clicks = [];
            // from = null;
            // to = null;
            current_state = GameState.HUMAN_TURN.THINKING;
            
            break;
        case GameState.AI_TURN:
            console.log("It's the AI's turn!");
            break;
    }
}

function getMousePos(canvas, evt) {
    var rect = canvas.getBoundingClientRect();
    return {
        x: evt.clientX - rect.left,
        y: evt.clientY - rect.top
    };
}

function setStatusText(txt) {
    statusText.hidden = false;
    statusText.innerText = txt;
}

