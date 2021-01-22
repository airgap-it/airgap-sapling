const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const CopyWebpackPlugin = require('copy-webpack-plugin')
const path = require('path')

const WasmPreloadPlugin = require('./plugins/WasmPreloadPlugin')

const nodeModules = path.resolve(__dirname, 'node_modules')

const baseConfig = {
  entry: path.resolve(__dirname, 'src/js/index.ts'),
  devtool: 'inline-source-map',
  target: ['es2020', 'node'],
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'index.js',
    libraryTarget: 'commonjs'
  },
  resolve: {
    extensions: ['.js', '.ts', '.wasm'],
    modules: [nodeModules],
    descriptionFiles: [path.resolve(__dirname, 'package.json')],
    symlinks: false
  },
  module: {
    rules: [
      { 
        test: /\.ts$/, 
        loader: 'ts-loader',
        exclude: nodeModules
      }
    ]
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname,
      outName: 'index',
      forceMode: 'production',
      extraArgs: '--target bundler --mode normal'
    }),
    new WasmPreloadPlugin({
      outputFile: 'index.js'
    })
  ],
  experiments: {
    asyncWebAssembly: true
  }
}

const prodConfig = baseConfig

const devConfig = {
  ...baseConfig,
  plugins: [
    ...baseConfig.plugins,
    new CopyWebpackPlugin({
      patterns: [
        {
          from: path.resolve(__dirname, 'package.json'),
          to: path.resolve(__dirname, 'dist')
        }
      ]
    })
  ]
}

module.exports = (env, argv) => {
  return (argv.mode === 'development') ? devConfig : prodConfig
}