import {
  MintProposed,
  Minted,
  MintChallenged
} from "../generated/ERC1155ChallengeableMint/ERC1155ChallengeableMint"
import { PendingMint } from "../generated/schema"
import { store } from '@graphprotocol/graph-ts'

export function handleMintProposed(event: MintProposed): void {  
  let entity = new PendingMint(event.params.id.toHex())
  entity.calldata = event.transaction.input
  entity.txn_hash = event.transaction.hash
  entity.token_id = event.params.id
  entity.to = event.params.to
  entity.timestamp = event.params.timestamp
  entity.disputed = false
  entity.save()
}

export function handleMinted(event: Minted): void {  
  store.remove("PendingMint", event.params.id.toHex())
}

export function handleMintChallenged(event: MintChallenged): void {  
  let entity = PendingMint.load(event.params.id.toHex())
  if (entity) {
    entity.disputed = true;
    entity.save()
  }
}
