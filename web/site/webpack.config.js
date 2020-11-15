const path = require('path')

const browserConfig = {
  entry: './index.js',
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: "development",
  devServer: {
    disableHostCheck: true,
    host: '0.0.0.0',
  },
}

const workerConfig = {
  entry: "./src/worker/mod.js",
  target: 'webworker',
  output: {
    path: path.resolve(__dirname, "dist") + "/src/worker",
    filename: "mod.js" // maybe full path?
  },
  mode: "development",
  devServer: {
    disableHostCheck: true,
    host: '0.0.0.0',
  },
}

module.exports = [browserConfig, workerConfig]

