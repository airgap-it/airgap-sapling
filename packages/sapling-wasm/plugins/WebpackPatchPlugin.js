const { Compilation } = require('webpack')
const { ReplaceSource } = require('webpack-sources')

class WebpackPatchPlugin {

  constructor(options) {
    this.targetFiles = options.targetFiles

    this.patchAssets = this.patchAssets.bind(this)
    this.patchRequire = this.patchRequire.bind(this)
  }

  apply(compiler) {
    compiler.hooks.make.tap('WebpackPatchPlugin', (compilation) => {
      compilation.hooks.processAssets.tap(
        {
          name: 'WebpackPatchPlugin',
          stage: Compilation.PROCESS_ASSETS_STAGE_DERIVED
        },
        (assets) => {
          this.patchAssets(assets, this.targetFiles, [this.patchRequire])
        }
      )
    })
  }

  patchAssets(assets, targetFiles, functions) {
    targetFiles.forEach((targetFile) => {
      const source = assets[targetFile]
      if (source) {
        const replaceSource = new ReplaceSource(source)
        functions.forEach((func) => {
          func(replaceSource)
        })

        assets[targetFile] = replaceSource
      }
    })
  }

  patchRequire(source) {
    const regex = RegExp(/__webpack_require__\(.*\)\(getStringFromWasm0\(arg0, arg1\)\)/, 'g')

    let match
    while ((match = regex.exec(source.source())) !== null) {
      source.replace(
        match.index,
        match.index + match[0].length,
        'require(getStringFromWasm0(arg0, arg1))'
      )

    }
  }
}

module.exports = WebpackPatchPlugin