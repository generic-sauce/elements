e2.load_tilemap = function(filename, callback) {
	// TODO allow changing map size
	var width = 128;
	var height = 72;

	var pixels = range(width).map(_ =>
					range(height).map(_ => [0, 0, 0, 255])
				 );

	var img = new Image();

	img.onload = function() {

		var canvas = document.createElement('canvas');
		var context = canvas.getContext('2d');

		context.drawImage(img, 0, 0);

		for (var x = 0; x < width; x++) {
			for (var y = 0; y < height; y++) {
				const map = context.getImageData(x, height - y - 1, 1, 1).data;
				const px = [map[0], map[1], map[2], map[3]];
				pixels[x][y] = px;
			}
		}

		const tilemap = {
			pixels: pixels,
			size: [width, height]
		};

		callback(tilemap);
	}
	img.src = "res/" + filename;
}

function range(n) {
	return [...Array(n).keys()];
}
