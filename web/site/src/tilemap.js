window.new_tilemap_image = function(filename) {
	var width = 128;
	var height = 72;
	let pixels = range(width).map(_ =>
					range(height).map(_ => [255, 255, 255, 255])
				 );
	for (var i = 0; i < width; i++) {
		pixels[i][10] = [0, 0, 0, 255];
	}

	return {
		pixels: pixels,
		size: [width, height]
	};
}

function range(n) {
	return [...Array(n).keys()];
}
