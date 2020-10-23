const path = require('path');
module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: "production",
  devServer: {
	disableHostCheck: true,
  	clientLogLevel: "info",
    host: '0.0.0.0',
  },
};
