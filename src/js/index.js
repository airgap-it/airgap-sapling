
import 'regenerator-runtime/runtime'

const saplingPromise = new Promise((resolve, reject) => {
  import('sapling-wasm')
  .then((sapling) => {
    resolve(sapling)
  })
  .catch((error) => {
    reject(`Could not load sapling-wasm: ${error}`)
  })
})

export async function getExtendedSpendingKey(seed, derivationPath) {
  const sapling = await saplingPromise
  let seedBuffer
  if (Buffer.isBuffer(seed)) {
    seedBuffer = seed
  } if (typeof seed === 'string') {
    seedBuffer = Buffer.from(seed, 'hex')
  } else {
    seedBuffer = Buffer.from(seed)
  }

  return Buffer.from(sapling.get_extended_spending_key(seedBuffer, derivationPath))
}