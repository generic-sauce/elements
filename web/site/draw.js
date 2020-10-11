import "./shader.js"

window.canvas = document.getElementById("main-canvas");

canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

window.init_drawing = function() {
	window.gl = canvas.getContext("webgl");
	init_drawing2(window.gl);
}

window.draw_world = function(world) {
	window.world = world;

	drawScene(window.gl, window.programInfo, window.buffers);
}
