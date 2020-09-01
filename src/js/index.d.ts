export interface SaplingPaymentAddress {
  index: Buffer
  raw: Buffer
}

export function getExtendedSpendingKey(seed: Buffer | Int8Array | string, derivationPath: string): Promise<Buffer>
export function getExtendedFullViewingKey(seed: Buffer | Int8Array | string, derivationPath: string): Promise<Buffer>
export function getPaymentAddressFromViewingKey(
  viewingKey: Buffer | Int8Array | string, 
  index?: Buffer | Int8Array | string | number
): Promise<SaplingPaymentAddress>