import * as tilemapmod from "./tilemap/mod.js";
import * as fluidmapmod from "./fluidmap/mod.js";
import * as playermod from "./player/mod.js";

export function init() {
	e2.canvas = document.getElementById("main-canvas");
	update_canvas_size()

	tilemapmod.init();
	fluidmapmod.init();
	playermod.init();
}

export function draw() {
	update_canvas_size()

	const gl = e2.gl;

	gl.clearColor(0.3, 0.0, 0.0, 1.0);
	gl.clear(gl.COLOR_BUFFER_BIT);

	gl.enable(gl.BLEND);
	gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

	tilemapmod.draw();
	fluidmapmod.draw();
	playermod.draw();
}

function update_canvas_size() {
	e2.canvas.width = window.innerWidth;
	e2.canvas.height = window.innerHeight;
	e2.gl = e2.canvas.getContext("webgl");
}
