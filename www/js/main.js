import {Game, start} from "../../pkg/index.js";
//triggers loading of resources in some way...
import * as resources from "./resources.js";

const canvas = document.getElementById("canvas")
var ctx = canvas.getContext("2d");

const game = Game.new(canvas, ctx);


//click listeners
//no jQuery this time
document.getElementsByClassName("key_arrow2")[0].addEventListener("click", event => {
  console.log("clicked up arrow");
  game.player_move(0,-1);
  console.log(game.pos_x() + game.pos_y());
});

document.getElementsByClassName("key_arrow4")[0].addEventListener("click", event => {
  console.log("clicked left arrow");
  game.player_move(-1,0);
  console.log(game.pos_x() + game.pos_y());
});

document.getElementsByClassName("key_arrow6")[0].addEventListener("click", event => {
  console.log("clicked right arrow");
  game.player_move(1,0);
  console.log(game.pos_x() + " " + game.pos_y());
});

document.getElementsByClassName("key_arrow8")[0].addEventListener("click", event => {
  console.log("clicked down arrow");
  game.player_move(0,1);
  console.log(game.pos_x() + " " + game.pos_y());
});

//load assets
window.resources.load([
    "gfx/human_m.png"
  ]);

//run wasm
window.resources.setReady(start)
