import * as tilemapmod from "./tilemap/mod.js";
import * as playermod from "./player/mod.js";

export function init() {
	e2.canvas = document.getElementById("main-canvas");
	e2.canvas.width = window.innerWidth;
	e2.canvas.height = window.innerHeight;
	e2.gl = e2.canvas.getContext("webgl");

	tilemapmod.init();
	playermod.init();
}

export function draw() {
	const gl = e2.gl;

	gl.clearColor(0.3, 0.0, 0.0, 1.0);
	gl.clear(gl.COLOR_BUFFER_BIT);

	tilemapmod.draw();
	playermod.draw();
}
