import { newMockEvent } from "matchstick-as"
import { ethereum, Address, BigInt } from "@graphprotocol/graph-ts"
import {
  ApprovalForAll,
  CreateERC1155_v1,
  OwnershipTransferred,
  SecondarySaleFees,
  SignerAdded,
  SignerRemoved,
  TransferBatch,
  TransferSingle,
  URI
} from "../generated/ERC1155ChallengeableMint/ERC1155ChallengeableMint"

export function createApprovalForAllEvent(
  _owner: Address,
  _operator: Address,
  _approved: boolean
): ApprovalForAll {
  let approvalForAllEvent = changetype<ApprovalForAll>(newMockEvent())

  approvalForAllEvent.parameters = new Array()

  approvalForAllEvent.parameters.push(
    new ethereum.EventParam("_owner", ethereum.Value.fromAddress(_owner))
  )
  approvalForAllEvent.parameters.push(
    new ethereum.EventParam("_operator", ethereum.Value.fromAddress(_operator))
  )
  approvalForAllEvent.parameters.push(
    new ethereum.EventParam("_approved", ethereum.Value.fromBoolean(_approved))
  )

  return approvalForAllEvent
}

export function createCreateERC1155_v1Event(
  creator: Address,
  name: string,
  symbol: string
): CreateERC1155_v1 {
  let createErc1155V1Event = changetype<CreateERC1155_v1>(newMockEvent())

  createErc1155V1Event.parameters = new Array()

  createErc1155V1Event.parameters.push(
    new ethereum.EventParam("creator", ethereum.Value.fromAddress(creator))
  )
  createErc1155V1Event.parameters.push(
    new ethereum.EventParam("name", ethereum.Value.fromString(name))
  )
  createErc1155V1Event.parameters.push(
    new ethereum.EventParam("symbol", ethereum.Value.fromString(symbol))
  )

  return createErc1155V1Event
}

export function createOwnershipTransferredEvent(
  previousOwner: Address,
  newOwner: Address
): OwnershipTransferred {
  let ownershipTransferredEvent = changetype<OwnershipTransferred>(
    newMockEvent()
  )

  ownershipTransferredEvent.parameters = new Array()

  ownershipTransferredEvent.parameters.push(
    new ethereum.EventParam(
      "previousOwner",
      ethereum.Value.fromAddress(previousOwner)
    )
  )
  ownershipTransferredEvent.parameters.push(
    new ethereum.EventParam("newOwner", ethereum.Value.fromAddress(newOwner))
  )

  return ownershipTransferredEvent
}

export function createSecondarySaleFeesEvent(
  tokenId: BigInt,
  recipients: Array<Address>,
  bps: Array<BigInt>
): SecondarySaleFees {
  let secondarySaleFeesEvent = changetype<SecondarySaleFees>(newMockEvent())

  secondarySaleFeesEvent.parameters = new Array()

  secondarySaleFeesEvent.parameters.push(
    new ethereum.EventParam(
      "tokenId",
      ethereum.Value.fromUnsignedBigInt(tokenId)
    )
  )
  secondarySaleFeesEvent.parameters.push(
    new ethereum.EventParam(
      "recipients",
      ethereum.Value.fromAddressArray(recipients)
    )
  )
  secondarySaleFeesEvent.parameters.push(
    new ethereum.EventParam("bps", ethereum.Value.fromUnsignedBigIntArray(bps))
  )

  return secondarySaleFeesEvent
}

export function createSignerAddedEvent(account: Address): SignerAdded {
  let signerAddedEvent = changetype<SignerAdded>(newMockEvent())

  signerAddedEvent.parameters = new Array()

  signerAddedEvent.parameters.push(
    new ethereum.EventParam("account", ethereum.Value.fromAddress(account))
  )

  return signerAddedEvent
}

export function createSignerRemovedEvent(account: Address): SignerRemoved {
  let signerRemovedEvent = changetype<SignerRemoved>(newMockEvent())

  signerRemovedEvent.parameters = new Array()

  signerRemovedEvent.parameters.push(
    new ethereum.EventParam("account", ethereum.Value.fromAddress(account))
  )

  return signerRemovedEvent
}

export function createTransferBatchEvent(
  _operator: Address,
  _from: Address,
  _to: Address,
  _ids: Array<BigInt>,
  _values: Array<BigInt>
): TransferBatch {
  let transferBatchEvent = changetype<TransferBatch>(newMockEvent())

  transferBatchEvent.parameters = new Array()

  transferBatchEvent.parameters.push(
    new ethereum.EventParam("_operator", ethereum.Value.fromAddress(_operator))
  )
  transferBatchEvent.parameters.push(
    new ethereum.EventParam("_from", ethereum.Value.fromAddress(_from))
  )
  transferBatchEvent.parameters.push(
    new ethereum.EventParam("_to", ethereum.Value.fromAddress(_to))
  )
  transferBatchEvent.parameters.push(
    new ethereum.EventParam(
      "_ids",
      ethereum.Value.fromUnsignedBigIntArray(_ids)
    )
  )
  transferBatchEvent.parameters.push(
    new ethereum.EventParam(
      "_values",
      ethereum.Value.fromUnsignedBigIntArray(_values)
    )
  )

  return transferBatchEvent
}

export function createTransferSingleEvent(
  _operator: Address,
  _from: Address,
  _to: Address,
  _id: BigInt,
  _value: BigInt
): TransferSingle {
  let transferSingleEvent = changetype<TransferSingle>(newMockEvent())

  transferSingleEvent.parameters = new Array()

  transferSingleEvent.parameters.push(
    new ethereum.EventParam("_operator", ethereum.Value.fromAddress(_operator))
  )
  transferSingleEvent.parameters.push(
    new ethereum.EventParam("_from", ethereum.Value.fromAddress(_from))
  )
  transferSingleEvent.parameters.push(
    new ethereum.EventParam("_to", ethereum.Value.fromAddress(_to))
  )
  transferSingleEvent.parameters.push(
    new ethereum.EventParam("_id", ethereum.Value.fromUnsignedBigInt(_id))
  )
  transferSingleEvent.parameters.push(
    new ethereum.EventParam("_value", ethereum.Value.fromUnsignedBigInt(_value))
  )

  return transferSingleEvent
}

export function createURIEvent(_value: string, _id: BigInt): URI {
  let uriEvent = changetype<URI>(newMockEvent())

  uriEvent.parameters = new Array()

  uriEvent.parameters.push(
    new ethereum.EventParam("_value", ethereum.Value.fromString(_value))
  )
  uriEvent.parameters.push(
    new ethereum.EventParam("_id", ethereum.Value.fromUnsignedBigInt(_id))
  )

  return uriEvent
}
