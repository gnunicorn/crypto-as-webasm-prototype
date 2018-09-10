// webpack.config.js
const path = require('path');

module.exports = {
  entry: "./html/index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
  },
  mode: "development"
};