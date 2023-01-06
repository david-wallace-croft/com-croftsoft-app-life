const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

module.exports = {
  devServer: {
    allowedHosts: ['localhost'],
    client: {
      logging: 'verbose',
      overlay: true,
      progress: true,
    },
    open: true,
    static: false,
  },
  entry: {
    index: './js/entry-index.js'
  },
  experiments: {
    asyncWebAssembly: true,
  },
  mode: 'production',
  output: {
    filename: 'com-croftsoft-app-life.js',
    path: path.resolve(__dirname, 'dist')
  },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, 'static')
    ]),
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ]
};
