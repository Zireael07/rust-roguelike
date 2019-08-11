import * as wasm from "../../pkg/index.js";
//triggers loading of resources in some way...
import * as resources from "./resources.js";

//load assets
window.resources.load([
    "gfx/human_m.png"
  ]);

//run wasm
window.resources.setReady(wasm.start)
