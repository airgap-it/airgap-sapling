const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const CopyWebpackPlugin = require('copy-webpack-plugin')
const path = require('path')

const WasmPreloadPlugin = require('./plugins/WasmPreloadPlugin')

const nodeModules = path.resolve(__dirname, 'node_modules')
const packageJson = path.resolve(__dirname, 'package.json')
const dist = path.resolve(__dirname, 'dist')

const baseConfig = {
  entry: path.resolve(__dirname, 'src/index.ts'),
  devtool: 'inline-source-map',
  target: ['es2020', 'node'],
  output: {
    path: dist,
    filename: 'index.js',
    libraryTarget: 'commonjs'
  },
  resolve: {
    extensions: ['.js', '.ts', '.wasm'],
    modules: [nodeModules],
    descriptionFiles: [packageJson],
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
      crateDirectory: path.resolve(__dirname, '../sapling'),
      outDir: path.resolve(__dirname, 'pkg'),
      outName: 'index',
      forceMode: 'production',
      extraArgs: '--target bundler --mode normal -- --features wasm_bindings'
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
          from: packageJson,
          to: dist
        }
      ]
    })
  ]
}

module.exports = (env, argv) => {
  return (argv.mode === 'development') ? devConfig : prodConfig
}