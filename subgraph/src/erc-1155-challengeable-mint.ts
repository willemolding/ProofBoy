import {
  MintProposed,
} from "../generated/ERC1155ChallengeableMint/ERC1155ChallengeableMint"
import { PendingMint } from "../generated/schema"

export function handleMintProposed(event: MintProposed): void {  
  let entity = new PendingMint(event.transaction.hash)
  entity.calldata = event.transaction.input
  entity.txn_hash = event.transaction.hash
  entity.token_id = event.params.id
  entity.to = event.params.to
  entity.timestamp = event.params.timestamp

  entity.save()
}
