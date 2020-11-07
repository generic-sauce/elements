export function load(filename, callback) {
	// TODO allow changing map size
	const width = 128;
	const height = 72;

	let pixels = range(width).map(_ =>
					range(height).map(_ => [0, 0, 0, 255])
				 );

	let img = new Image();

	img.onload = function() {

		let canvas = document.createElement('canvas');
		let context = canvas.getContext('2d');

		context.drawImage(img, 0, 0);

		for (let x = 0; x < width; x++) {
			for (let y = 0; y < height; y++) {
				const map = context.getImageData(x, height - y - 1, 1, 1).data;
				const px = [map[0], map[1], map[2], map[3]];
				pixels[x][y] = px;
			}
		}

		callback({
			pixels,
			size: [width, height]
		});
	};

	img.src = "res/" + filename;
}

function range(n) {
	return [...Array(n).keys()];
}
