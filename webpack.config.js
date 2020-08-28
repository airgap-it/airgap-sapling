const CopyWebpackPlugin = require('copy-webpack-plugin')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

const path = require('path')

module.exports = {
  entry: path.resolve(__dirname, 'src/js/index.js'),
  target: 'node',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'index.js',
    libraryTarget: 'commonjs'
  },
  resolve: {
    extensions: ['.js', '.ts'],
    modules: [path.resolve(__dirname, 'node_modules')],
    descriptionFiles: [path.resolve(__dirname, 'package.json')],
    symlinks: false,
    alias: {
      'wasm-sapling': path.resolve(__dirname, 'pkg/wasm-sapling.js')
    }
  },
  module: {
    rules: [
      {
        use: {
          loader: 'babel-loader'
        },
        exclude: [
          /\.wasm$/
        ]
      }
    ]
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        {
          from: path.resolve(__dirname, 'src/js/index.d.ts'),
          to: path.resolve(__dirname, 'dist')
        }
      ]
    }),
    new WasmPackPlugin({
      crateDirectory: __dirname,
      outName: 'wasm-sapling'
    })
  ]
};