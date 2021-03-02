const { Compilation, RuntimeGlobals } = require('webpack')
const { OriginalSource, ReplaceSource } = require('webpack-sources')

const extensions = {
  wasmModule: '.module.wasm',
  wasmBase64Asset: '_wasm.js'
}

class WasmPreloadPlugin {
  constructor(options) {
    this.outputFile = options.outputFile

    this.fixWbgRequire = this.fixWbgRequire.bind(this)
    this.fixWasmImport = this.fixWasmImport.bind(this)
  }

  apply(compiler) {
    compiler.hooks.make.tap('WasmPreloadPlugin', (compilation) => {
        compilation.hooks.processAssets.tap(
          {
            name: 'WasmPreloadPlugin',
            stage: Compilation.PROCESS_ASSETS_STAGE_DERIVED
          }, 
          (assets) => {
            const outputAsset = assets[this.outputFile]
            if (outputAsset) {
              this.prepareLoadedWasmSources(assets, compilation)
              assets[this.outputFile] = this.adjustSource(assets, outputAsset, [this.fixWbgRequire, this.fixWasmImport])
            }
          }
        )
      })
  }

  prepareLoadedWasmSources(assets, compilation) {
    Object.entries(assets)
      .filter(([key, _]) => key.endsWith(extensions.wasmModule))
      .map(([key, source]) => [key.replace(extensions.wasmModule, ''), source.source()])
      .forEach(([key, source]) => {
        const wasmBase64Name = key + extensions.wasmBase64Asset
        const wasmBase64Source = `module.exports = Buffer.from('${source.toString('base64')}', 'base64')`
        const wasmBase64Asset = new OriginalSource(wasmBase64Source, wasmBase64Name)

        compilation.emitAsset(wasmBase64Name, wasmBase64Asset)
        compilation.deleteAsset(key + extensions.wasmModule)
      })
  }

  adjustSource(assets, source, functions) {
    const replaceSource = new ReplaceSource(source)
    functions.forEach((func) => {
      func(assets, replaceSource)
    })
    
    return replaceSource
  }

  fixWbgRequire(assets, source) {
    const regex = RegExp(/getObject\((\w+)\).require\((\w+)\((\w+),\s*(\w+)\)\)/, 'g')

    let n = 0
    let match
    while ((match = regex.exec(source.source())) !== null) {
      source.replace(
        match.index, 
        match.index + match[0].length, 
        `require(${match[2]}(${match[3]}, ${match[4]}));`, 
        `__wbg_require__#${n}`
      )
      n++
    }
  }

  fixWasmImport(assets, source) {
    const wasmModuleHashes = Object.keys(assets)
      .filter((key) => key.endsWith(extensions.wasmModule) || key.endsWith(extensions.wasmBase64Asset))
      .map((key) => key.replace(extensions.wasmModule, '').replace(extensions.wasmBase64Asset, ''))

    const regex = RegExp(`${RuntimeGlobals.instantiateWasm} = \((.*)\) => {.*};`, 'gs')

    let n = 0
    let match
    while ((match = regex.exec(source.source())) !== null) {
      source.replace(
        match.index,
        match.index + match[0].length,
        this.prepareInstantiateWasmFunction(match[1], wasmModuleHashes),
        `${RuntimeGlobals.instantiateWasm}#${n}`
      )
    }
  }

  prepareInstantiateWasmFunction(args, wasmModuleHashes) {
    const exportsArg = 'exports'
    const wasmModuleHashArg = 'wasmModuleHash'
    const importsObjArg = 'importsObj'
    if (!args.includes(exportsArg) || !args.includes(wasmModuleHashArg) || !args.includes(importsObjArg)) {
      throw new Error(`Unknown '${RuntimeGlobals.instantiateWasm}' signature`)
    }

    const wasmRequires = wasmModuleHashes
      .map((hash) => [hash, `() => require('./${hash}${extensions.wasmBase64Asset}')`])
      .reduce((obj, next) => Object.assign(obj, { [next[0]]: next[1] }), {})

    return `${RuntimeGlobals.instantiateWasm} = ${args} => {
      var requires = ${this.objectToString(wasmRequires)}
      var buffer = requires[${wasmModuleHashArg}] ? requires[${wasmModuleHashArg}]() : Buffer.alloc(0)

      return WebAssembly.instantiate(buffer, ${importsObjArg})
        .then((res) => Object.assign(${exportsArg}, res.instance.exports));
    };\n`
  }

  objectToString(obj) {
    const str = Object.entries(obj)
      .map(([key, value]) => `'${key}': ${value}`)
      .reduce((concat, next) => concat + ',' + next)

    return `{${str}}`
  }
}

module.exports = WasmPreloadPlugin