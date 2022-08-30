const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const CopyWebpackPlugin = require('copy-webpack-plugin')
const path = require('path')

const WasmPreloadPlugin = require('./plugins/WasmPreloadPlugin')
const WebpackPatchPlugin = require('./plugins/WebpackPatchPlugin')

const nodeModules = path.resolve(__dirname, 'node_modules')
const packageJson = path.resolve(__dirname, 'package.json')
const dist = path.resolve(__dirname, 'dist')

const commonConfig = {
  entry: {
    browser: path.resolve(__dirname, 'src/index.ts'),
    node: path.resolve(__dirname, 'src/index.ts')
  },
  devtool: 'inline-source-map',
  target: ['es2020', 'node'],
  output: {
    path: dist,
    filename: 'index.[name].js',
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
    ],
    exprContextCritical: false
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
      targetFiles: ['index.browser.js', 'index.node.js']
    }),
    new WebpackPatchPlugin({
        targetFiles: ['index.node.js']
    })
  ],
  experiments: {
    asyncWebAssembly: true
  }
}

const prodConfig = commonConfig

const devConfig = {
  ...commonConfig,
  plugins: [
    ...commonConfig.plugins,
    new CopyWebpackPlugin({
      patterns: [
        {
          from: packageJson,
          to: dist,
          transform(content) {
            return content.toString()
              .replace(/\/dist/g, ".")
              .replace(/dist\//g, "")
          }
        }
      ]
    })
  ]
}

module.exports = (env, argv) => {
  return (argv.mode === 'development') ? devConfig : prodConfig
}