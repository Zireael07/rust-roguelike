//simple import for when we have nothing on JS side
import("../pkg/index.js").catch(console.error);


// A dependency graph that contains any wasm must all be imported
// asynchronously. This file does the single async import, so
// that no one else needs to worry about it again.
//import("./main.js")
//  .catch(e => console.error("Error importing `main.js`:", e));