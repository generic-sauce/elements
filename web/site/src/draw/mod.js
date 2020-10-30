import * as fluidmapmod from "./fluidmap/mod.js";
import * as tilemapmod from "./tilemap/mod.js";
import * as playermod from "./player/mod.js";

export function init() {
	e2.canvas = document.getElementById("main-canvas");
	window.onresize = update_canvas_size
	update_canvas_size()

	e2.gl.enable(e2.gl.BLEND);
	e2.gl.blendFunc(e2.gl.SRC_ALPHA, e2.gl.ONE_MINUS_SRC_ALPHA);
	e2.gl.clearColor(0.8, 0.8, 1.0, 1.0);

	fluidmapmod.init();
	tilemapmod.init();
	playermod.init();
}

export function draw() {
	const gl = e2.gl;

	gl.clear(gl.COLOR_BUFFER_BIT);

	fluidmapmod.draw();
	tilemapmod.draw();
	playermod.draw();
}

function update_canvas_size() {
	e2.canvas.width = window.innerWidth;
	e2.canvas.height = window.innerHeight;
	e2.gl = e2.canvas.getContext("webgl");
	e2.gl.viewport(0, 0, e2.canvas.width, e2.canvas.height);
}
