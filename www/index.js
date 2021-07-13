import { Game, Board, BrdIdx, Painter, Team, init_wasm, Moveable, SquareState, Square } from "draught";
// import { memory } from "draught/draught_bg.wasm";

///////////////////
//    CONSTS
///////////////////

const CANVAS_WIDTH = 720;
const CANVAS_HEIGHT = 720;

var BOARD_WIDTH = 8;
var BOARD_HEIGHT = 8;

var PIECE_ROWS = 3;
var SEARCH_DEPTH = 4;

const STATUS_TIMEOUT = 3000;
const WON_TIMEOUT = 3000;

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
const nodeCountText = document.getElementById("node-count");
const winningText = document.getElementById("winning-p");

const startBtn = document.getElementById("startBtn");
startBtn.onclick = start_game;

let wonTimeout = null;
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
        Math.floor((mousepos.y / canvas.clientHeight) * BOARD_HEIGHT),
        Math.floor((mousepos.x / canvas.clientWidth) * BOARD_WIDTH),
    );
    // console.log(cell);
    process_canvas_click(cell);
});

////////////////
//   FUNCS
////////////////

function start_game() {
    game = new Game(BOARD_WIDTH, BOARD_HEIGHT, PIECE_ROWS, Team.Black, SEARCH_DEPTH);
    painter = new Painter(CANVAS_WIDTH, CANVAS_HEIGHT, "game-canvas");
    game.set_painter(painter);
    game.draw();

    clearInterval(wonTimeout);
    updateTeamText();
    updateWinningText();
    clicks = [];
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

                        if (aiCheckBox.checked) {
                            game.ai_move();
                            nodeCountText.innerText = `searched ${game.last_node_count.toLocaleString("en-GB")} possible moves`;
                        }

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
    updateWinningText();
    checkWon();
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

function updateWinningText(){
    
    switch(game.winning()) {
        case undefined:
            winningText.innerText = "";
            break;
        case Team.White:
            winningText.innerText = "👑 White 👑";
            break;
        case Team.Black:
            winningText.innerText = "👑 Black 👑";
            break;
    }
}

function checkWon() {

    switch(game.has_won()) {
        case undefined:
            break;
        case Team.White:
            setStatus("You Lost!");
            wonTimeout = setInterval(() => {
                start_game();
            }, WON_TIMEOUT);
            break;
        case Team.Black:
            setStatus("You Won!", "success");
            wonTimeout = setInterval(() => {
                start_game();
            }, WON_TIMEOUT);
            break;
    }
}

////////////////
//     UI
////////////////

const widthBox = document.getElementById("width");
/**
 * Handler for width input box change, start a new game
 */
const onWidth = () => {

    BOARD_WIDTH = parseInt(widthBox.value);
    start_game();
}
widthBox.onchange = onWidth;
widthBox.value = 8;

const heightBox = document.getElementById("height");
/**
 * Handler for height input box change, start a new game
 */
const onHeight = () => {

    BOARD_HEIGHT = parseInt(heightBox.value);
    pieceRowsBox.max =  (BOARD_HEIGHT / 2) - 1;
    start_game();
}
heightBox.onchange = onHeight;
heightBox.value = 8;

const pieceRowsBox = document.getElementById("play_rows");
/**
 * Handler for piece rows input box change, start a new game
 */
const onPieceRows = () => {

    PIECE_ROWS = parseInt(pieceRowsBox.value);
    start_game();
}
pieceRowsBox.onchange = onPieceRows;
pieceRowsBox.value = 3;

const aiSearchDepthBox = document.getElementById("ai_search_depth");
/**
 * Handler for AI search depth input box change, start a new game
 */
const onAISearchDepth = () => {

    SEARCH_DEPTH = parseInt(aiSearchDepthBox.value);
    game.set_search_depth(SEARCH_DEPTH);

    if(SEARCH_DEPTH > 4) {
        setStatus("This increases thinking time exponentially, be careful (probably don't go past 6)", "warning");
    }
}
aiSearchDepthBox.onchange = onAISearchDepth;
aiSearchDepthBox.value = 4;

const aiCheckBox = document.getElementById("ai-checkbox");
/**
 * Handler for height input box change, get a new universe of given size
 */
const onAICheck = () => {
    
}
aiCheckBox.onchange = onAICheck;
// aiCheckBox.checked = true;