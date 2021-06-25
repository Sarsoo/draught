import { init_game } from "draught";
import { memory } from "draught/draught_bg.wasm";

// let PLAY = true;
// let PLAY = false;
init_game();

const canvas = document.getElementById("game-canvas");
const ctx = canvas.getContext('2d');
