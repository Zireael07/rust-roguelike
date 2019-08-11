const canvas = document.getElementById("canvas")
var ctx = canvas.getContext("2d");

// renderer functions which rely on resources (since the resources themselves are on JS side)

export function renderPlayer(x,y){
    renderGfxTile(window.resources.get("gfx/human_m.png"), x,y);
}
export function renderGfxTile(img, x, y) {
    ctx.drawImage(img, x*32, y*32);
}