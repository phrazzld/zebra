const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');
const TerserPlugin = require('terser-webpack-plugin');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "production",
  experiments: {
    asyncWebAssembly: true,  // Enable WebAssembly as async modules
    syncWebAssembly: true,   // Enable WebAssembly as sync modules
  },
  optimization: {
    minimize: true,
    minimizer: [new TerserPlugin()],
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: "index.html" },
        { from: "manifest.json" },
        { from: "favicon.ico" },
        { from: "android-chrome-192x192.png" },
        { from: "android-chrome-512x512.png" }
      ]
    })
  ],
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "webassembly/async"
      }
    ]
  },
  devServer: {
    static: {
      directory: path.join(__dirname, 'dist'),
    },
    compress: true,
    port: 8080,
    hot: true
  }
};
