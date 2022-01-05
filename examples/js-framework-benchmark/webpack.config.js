const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "bundled-dist");

module.exports = {
  mode: "production",
  stats: "errors-warnings",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    publicPath: "bundled-dist/",
    filename: "[name].js"
  },
  experiments: {
    syncWebAssembly: true
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname,
      outName: "index"
    })
  ]
};
