export interface SaplingPaymentAddress {
  index: Buffer
  raw: Buffer
}

export interface SaplingSpendDescription {
  cv: Buffer
  rt: Buffer
  nf: Buffer
  rk: Buffer
  zkproof: Buffer
  spendAuthSig: Buffer
}

export interface SaplingTransaction {
  spendDescriptions: SaplingSpendDescription[]
  outputDescriptions: SaplingOutputDescription[]
}

export function getExtendedSpendingKey(seed: Buffer | Int8Array | string, derivationPath: string): Promise<Buffer>
export function getExtendedFullViewingKey(seed: Buffer | Int8Array | string, derivationPath: string): Promise<Buffer>

export function getPaymentAddressFromViewingKey(
  viewingKey: Buffer | Int8Array | string, 
  index?: Buffer | Int8Array | string | number | undefined
): Promise<SaplingPaymentAddress>
export function getNextPaymentAddressFromViewingKey(
  viewingKey: Buffer | Int8Array | string,
  index: Buffer | Int8Array | string | number
): Promise<SaplingPaymentAddress>

export function prepareSpendingDescription(
  viewingKey: Buffer | Int8Array | string
)

export function prepareOutputDescription(
  viewingKey: Buffer | Int8Array | string, 
  destination: SaplingPaymentAddress | Buffer | Int8Array | string, 
  value: string | number,
  memo?: Buffer | Int8Array | string | undefined
): Promise<Buffer>