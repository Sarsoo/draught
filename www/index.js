import { Game, Board, BrdIdx, Painter, Team, init_wasm, Moveable, SquareState, Square } from "draught";
// import { memory } from "draught/draught_bg.wasm";

///////////////////
//    CONSTS
///////////////////

const CANVAS_WIDTH = 480;
const CANVAS_HEIGHT = 480;

const BOARD_WIDTH = 8;
const BOARD_HEIGHT = 8;

const PIECE_ROWS = 3;

const STATUS_TIMEOUT = 3000;

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
const statusAlert = document.getElementById("status-d");
const teamText = document.getElementById("team-p");

const startBtn = document.getElementById("startBtn");
startBtn.onclick = start_game;

let statusTimeout = null;
let setStatus = setStatusAlert;

let current_state = GameState.HUMAN_TURN.THINKING;

let game = null;
let painter = null;

let clicks = [];

start_game();

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
    painter = new Painter(CANVAS_WIDTH, CANVAS_HEIGHT, "game-canvas");
    game.set_painter(painter);
    game.draw();

    updateTeamText();
    current_state = GameState.HUMAN_TURN.THINKING;
}

function process_canvas_click(cell_coord) {

    switch(current_state) {
        // first click of a move
        case GameState.HUMAN_TURN.THINKING:
            if (game.current_cell_state(cell_coord).state != SquareState.Occupied ) {
                return;
            }

            if (game.current_cell_state(cell_coord).occupant.team != game.current_turn() ) {
                return;
            }

            // console.log("Your turn, first piece picked");

            clicks.push(cell_coord);
            current_state = GameState.HUMAN_TURN.FROM_SELECTED;
            game.set_selected(cell_coord);
            game.draw();
            
            break;
            
        // second click of a move
        case GameState.HUMAN_TURN.FROM_SELECTED:

            // second click is different to first, process as move
            // otherwise, will skip straight to clean up (clear selected and clicks) 
            if (!clicks[0].eq(cell_coord)) {

                if (game.current_cell_state(cell_coord).state != SquareState.Empty ) {
                    return;
                }
    
                // console.log("Your turn, first piece already picked, picking second");
    
                clicks.push(cell_coord);
    
                if (clicks.length != 2) {
                    setStatus(`Error: wrong number of clicks to process ${clicks.length}`);
                    console.error(`Error: wrong number of clicks to process ${clicks.length}`);
    
                    return;
                }

                let status = game.make_move(clicks[0], clicks[1]);

                switch(status) {
                    case Moveable.Allowed:
                        break;
                    case Moveable.IllegalTrajectory:
                        setStatus("You can't move like that!");
                        break;
                    case Moveable.JumpingSameTeam:
                        setStatus("You can't jump your own piece!");
                        break;
                    case Moveable.NoJumpablePiece:
                        setStatus("There's nothing to jump!");
                        break;
                    case Moveable.OccupiedDest:
                        setStatus("There's a piece there!");
                        break;
                    case Moveable.OutOfBounds:
                        setStatus("That square's not on the board! (how have you managed that?)");
                        break;
                    case Moveable.UnoccupiedSrc:
                        setStatus("There's no piece to move!");
                        break;
                    case Moveable.Unplayable:
                        setStatus("That's not a playable square!");
                        break;
                    case Moveable.WrongTeamSrc:
                        setStatus("That's not your piece!");
                        break;
                }
                
            }

            game.clear_selected();
            game.draw();
            clicks = [];
            current_state = GameState.HUMAN_TURN.THINKING;
            
            break;
        case GameState.AI_TURN:
            console.log("It's the AI's turn!");
            break;
    }

    updateTeamText();
}

function getMousePos(canvas, evt) {
    var rect = canvas.getBoundingClientRect();
    return {
        x: evt.clientX - rect.left,
        y: evt.clientY - rect.top
    };
}

function setStatusText(txt, hide = true) {
    if(statusTimeout != null) {
        clearInterval(statusTimeout);
    }

    statusText.innerText = txt;

    if(hide) {
        statusTimeout = setTimeout(() => {
            statusText.innerText = "";
        }, STATUS_TIMEOUT);
    }
}

function setStatusAlert(txt, alertType = "danger", hide = true) {
    if(statusTimeout != null) {
        clearInterval(statusTimeout);
    }

    statusAlert.className = `alert alert-${alertType}`;
    statusAlert.innerText = txt;
    statusAlert.hidden = false;

    if(hide) {
        statusTimeout = setTimeout(() => {
            statusAlert.hidden = true;
        }, STATUS_TIMEOUT);
    }
}

function updateTeamText(){
    let team = game.current_turn();
    switch(team) {
        case Team.White:
            teamText.innerText = "⚪ White ⚪";
            break;
        case Team.Black:
            teamText.innerText = "🔴 Black 🔴";
            break;
    }
}
