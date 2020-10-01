const CopyWebpackPlugin = require('copy-webpack-plugin')
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')

const path = require('path')

module.exports = {
  entry: path.resolve(__dirname, 'src/js/index.ts'),
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
    symlinks: false
  },
  module: {
    rules: [
      { 
        test: /\.ts$/, 
        loader: "awesome-typescript-loader" 
      },

      { 
        test: /\.js$/, 
        loader: "source-map-loader" 
      }
    ]
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        {
          from: path.resolve(__dirname, 'package.json'),
          to: path.resolve(__dirname, 'dist')
        }
      ]
    }),
    new WasmPackPlugin({
      crateDirectory: __dirname,
      outName: 'index',
      forceMode: 'production'
    })
  ]
};