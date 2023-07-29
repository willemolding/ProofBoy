
export interface NftMetadata {
    name: string,
    description: string,
    image: string,
    attributes: Array<{
      trait_type: string,
      value: string
    }>
  }
  
export interface ProofBoyData {
    data: NftMetadata,
    journal: Uint8Array
}
