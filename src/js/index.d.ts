export interface SaplingPaymentAddress {
  index: Buffer
  raw: Buffer
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

export function withProvingContext<T>(action: (context: Object) => T): T

export function prepareSpendDescription(
  context: Object,
  spendingKey: Buffer | Int8Array | string,
  address: SaplingPaymentAddress | Buffer | Int8Array | string,
  rcm: Buffer | Int8Array | string,
  ar: Buffer | Int8Array | string,
  value: string | number,
  anchor: Buffer | Int8Array | string,
  merklePath: Buffer | Int8Array | string,
  position: number,
  provingKey: Buffer | Int8Array | string,
  verifyingKey: Buffer | Int8Array | string
): Promise<Buffer>

export function prepareOutputDescription(
  context: Object,
  viewingKey: Buffer | Int8Array | string, 
  destination: SaplingPaymentAddress | Buffer | Int8Array | string, 
  rcm: Buffer | Int8Array | string,
  value: string | number,
  provingKey: Buffer | Int8Array | string,
  memo?: Buffer | Int8Array | string | undefined
): Promise<Buffer>