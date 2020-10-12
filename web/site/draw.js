window.canvas = document.getElementById("main-canvas");

canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

window.init_drawing = function() {
	window.gl = canvas.getContext("webgl");

	const vsSource = `
		attribute vec2 aVertexPosition;
		attribute vec2 aTextureCoord;

		varying highp vec2 vTextureCoord;

		void main() {
			gl_Position = vec4(aVertexPosition, 0, 1);
			vTextureCoord = aTextureCoord;
		}
	`;

	const fsSource = `
		varying highp vec2 vTextureCoord;

		uniform sampler2D uSampler;

		void main() {
			// gl_FragColor = vec4(texture2D(uSampler, vTextureCoord).rgb, 1.0);
			gl_FragColor = texture2D(uSampler, vTextureCoord);
		}
	`;

	const shaderProgram = initShaderProgram(gl, vsSource, fsSource);
	window.programInfo = {
		program: shaderProgram,
		attribLocations: {
			vertexPosition: gl.getAttribLocation(shaderProgram, 'aVertexPosition'),
			textureCoord: gl.getAttribLocation(shaderProgram, 'aTextureCoord'),
		},
		uniformLocations: {
			uSampler: gl.getUniformLocation(shaderProgram, 'uSampler'),
		},
	};

	const textureCoordBuffer = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, textureCoordBuffer);

	const r = [
		0.0,  0.0,
		1.0,  0.0,
		0.0,  1.0,
		1.0,  1.0,
	];
	const tc = [r.slice(0, 6), r.slice(2,8), r.slice(0, 6), r.slice(2,8)].flat();
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(tc), gl.STATIC_DRAW);
	window.buffers = {
		position: gl.createBuffer(),
		textureCoord: textureCoordBuffer,
	};
	window.texture = loadTexture(gl, './file.png');
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

	const r0 = player_rect(0);
	const r1 = player_rect(1);
	const positions = [r0.slice(0, 6), r0.slice(2,8), r1.slice(0, 6), r1.slice(2,8)].flat();

	gl.bufferData(gl.ARRAY_BUFFER,
								new Float32Array(positions),
								gl.STATIC_DRAW);

	gl.clearColor(0.3, 0.0, 0.0, 1.0);
	gl.clear(gl.COLOR_BUFFER_BIT);

	{
    const numComponents = 2;
    const type = gl.FLOAT;
    const normalize = false;
    const stride = 0;
    const offset = 0;
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

  {
    const numComponents = 2;
    const type = gl.FLOAT;
    const normalize = false;
    const stride = 0;
    const offset = 0;
    gl.bindBuffer(gl.ARRAY_BUFFER, buffers.textureCoord);
    gl.vertexAttribPointer(
        programInfo.attribLocations.textureCoord,
        numComponents,
        type,
        normalize,
        stride,
        offset);
    gl.enableVertexAttribArray(
        programInfo.attribLocations.textureCoord);
  }

	gl.useProgram(programInfo.program);
	gl.activeTexture(gl.TEXTURE0);
	gl.bindTexture(gl.TEXTURE_2D, window.texture);
	gl.uniform1i(programInfo.uniformLocations.uSampler, 0);

	{
		const offset = 0;
		const vertexCount = 12;
		gl.drawArrays(gl.TRIANGLES, offset, vertexCount);
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

function loadTexture(gl, url) {
  const texture = gl.createTexture();
  gl.bindTexture(gl.TEXTURE_2D, texture);

  // Because images have to be download over the internet
  // they might take a moment until they are ready.
  // Until then put a single pixel in the texture so we can
  // use it immediately. When the image has finished downloading
  // we'll update the texture with the contents of the image.
  const level = 0;
  const internalFormat = gl.RGBA;
  const width = 1;
  const height = 1;
  const border = 0;
  const srcFormat = gl.RGBA;
  const srcType = gl.UNSIGNED_BYTE;
  const pixel = new Uint8Array([0, 0, 255, 255]);  // opaque blue
  gl.texImage2D(gl.TEXTURE_2D, level, internalFormat,
                width, height, border, srcFormat, srcType,
                pixel);

  const image = new Image();
  image.onload = function() {
    gl.bindTexture(gl.TEXTURE_2D, texture);
    gl.texImage2D(gl.TEXTURE_2D, level, internalFormat,
                  srcFormat, srcType, image);

    // WebGL1 has different requirements for power of 2 images
    // vs non power of 2 images so check if the image is a
    // power of 2 in both dimensions.
    if (isPowerOf2(image.width) && isPowerOf2(image.height)) {
       // Yes, it's a power of 2. Generate mips.
       gl.generateMipmap(gl.TEXTURE_2D);
    } else {
       // No, it's not a power of 2. Turn off mips and set
       // wrapping to clamp to edge
       gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
       gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
       gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    }
  };
  image.src = url;

  return texture;
}

function isPowerOf2(value) {
  return (value & (value - 1)) == 0;
}
