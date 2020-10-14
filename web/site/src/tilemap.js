window.new_tilemap_image = function(filename) {
	// TODO allow changing map size
	var width = 128;
	var height = 72;

	let pixels = range(width).map(_ =>
					range(height).map(_ => [255, 255, 255, 255])
				 );

	const pfn = pixel_fn("res/" + filename);

	for (var x = 0; x < width; x++) {
		for (var y = 0; y < height; y++) {
			const px = pfn(x, y);
			pixels[x][y] = px;
		}
	}

	return {
		pixels: pixels,
		size: [width, height]
	};
}

function range(n) {
	return [...Array(n).keys()];
}


function pixel_fn(url) {
	var img = new Image();
	img.src = url;
	var canvas = document.createElement('canvas');
	var context = canvas.getContext('2d');
	context.drawImage(img, 0, 0);
	return function(x, y) {
		const map = context.getImageData(x, y, 1, 1).data;
		const res = [map[0], map[1], map[2], map[3]];
		if (res[3] == 0) {
			console.log("error: out of range!", x, y);
		}
		return res;
	}
}
