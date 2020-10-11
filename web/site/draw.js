window.canvas = document.getElementById("main-canvas");

canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

window.init_drawing = function() {
	window.gl = canvas.getContext("webgl");

	const vsSource = `
		attribute vec4 aVertexPosition;

		void main() {
			gl_Position = aVertexPosition;
		}
	`;

	const fsSource = `
		void main() {
			gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
		}
	`;

	const shaderProgram = initShaderProgram(gl, vsSource, fsSource);
	window.programInfo = {
		program: shaderProgram,
		attribLocations: {
			vertexPosition: gl.getAttribLocation(shaderProgram, 'aVertexPosition'),
		},
		uniformLocations: {
		},
	};

	window.buffers = { position: gl.createBuffer() };
}

window.draw_world = function(world, constants) {
	window.world = world;
	window.constants = constants;

	drawScene(window.gl, window.programInfo, window.buffers);
}

function max_game_point() {
	return world.tilemap.size
		.map(x => x * 256);
}

function game_to_screen_point(p) {
	return [p[0] / max_game_point()[0], p[1] / max_game_point()[1]]
		.map(x => 2*x - 1);
}

function player_rect(i) {
	const lb = window.world.players[i].left_bot;
	const s = window.constants.player_size;
	const game_rect = [
		[lb[0]       , lb[1]],
		[lb[0] + s[0], lb[1]],
		[lb[0]       , lb[1] + s[1]],
		[lb[0] + s[0], lb[1] + s[1]],
	];
	let screen_rect = game_rect.map(game_to_screen_point);
	return [
		screen_rect[0][0], screen_rect[0][1],
		screen_rect[1][0], screen_rect[1][1],
		screen_rect[2][0], screen_rect[2][1],
		screen_rect[3][0], screen_rect[3][1],
	];

}

window.drawScene = function(gl, programInfo, buffers) {

	gl.bindBuffer(gl.ARRAY_BUFFER, buffers.position);

	// Now create an array of positions for the square.

	const positions = player_rect(0);

	// Now pass the list of positions into WebGL to build the
	// shape. We do this by creating a Float32Array from the
	// JavaScript array, then use it to fill the current buffer.

	gl.bufferData(gl.ARRAY_BUFFER,
								new Float32Array(positions),
								gl.STATIC_DRAW);

	gl.clearColor(0.0, 0.0, 0.0, 1.0);
	gl.clear(gl.COLOR_BUFFER_BIT);

	{
		const numComponents = 2;	// pull out 2 values per iteration
		const type = gl.FLOAT;		// the data in the buffer is 32bit floats
		const normalize = false;	// don't normalize
		const stride = 0;				 // how many bytes to get from one set of values to the next
															// 0 = use type and numComponents above
		const offset = 0;				 // how many bytes inside the buffer to start from
		gl.bindBuffer(gl.ARRAY_BUFFER, buffers.position);
		gl.vertexAttribPointer(
				programInfo.attribLocations.vertexPosition,
				numComponents,
				type,
				normalize,
				stride,
				offset);
		gl.enableVertexAttribArray(
				programInfo.attribLocations.vertexPosition);
	}

	gl.useProgram(programInfo.program);

	{
		const offset = 0;
		const vertexCount = 4;
		gl.drawArrays(gl.TRIANGLE_STRIP, offset, vertexCount);
	}
}


function initShaderProgram(gl, vsSource, fsSource) {
	const vertexShader = loadShader(gl, gl.VERTEX_SHADER, vsSource);
	const fragmentShader = loadShader(gl, gl.FRAGMENT_SHADER, fsSource);

	// Create the shader program

	const shaderProgram = gl.createProgram();
	gl.attachShader(shaderProgram, vertexShader);
	gl.attachShader(shaderProgram, fragmentShader);
	gl.linkProgram(shaderProgram);

	// If creating the shader program failed, alert

	if (!gl.getProgramParameter(shaderProgram, gl.LINK_STATUS)) {
		alert('Unable to initialize the shader program: ' + gl.getProgramInfoLog(shaderProgram));
		return null;
	}

	return shaderProgram;
}

//
// creates a shader of the given type, uploads the source and
// compiles it.
//
function loadShader(gl, type, source) {
	const shader = gl.createShader(type);

	// Send the source to the shader object

	gl.shaderSource(shader, source);

	// Compile the shader program

	gl.compileShader(shader);

	// See if it compiled successfully

	if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
		alert('An error occurred compiling the shaders: ' + gl.getShaderInfoLog(shader));
		gl.deleteShader(shader);
		return null;
	}

	return shader;
}
