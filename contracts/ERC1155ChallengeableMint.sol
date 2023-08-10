//SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import { Base64 } from "@openzeppelin/contracts/utils/Base64.sol";
import { ERC1155URIStorage, ERC1155 } from "@openzeppelin/contracts/token/ERC1155/extensions/ERC1155URIStorage.sol";
import { IDisputeGameFactory } from "@eth-optimism/contracts-bedrock/src/dispute/interfaces/IDisputeGameFactory.sol";
import { IDisputeGame } from "@eth-optimism/contracts-bedrock/src/dispute/interfaces/IDisputeGame.sol";
import { GameTypes, Claim, GameStatus } from "@eth-optimism/contracts-bedrock/src/libraries/DisputeTypes.sol";

/**
* @title ERC1155ChallengeableMint
* @dev This contract allows for the creation of ERC1155 tokens through a two stage minting process
*
*      The first stage is a proposal stage where a user can propose a new token to be minted
*      during a challenge period anyone who disputes the validity of the token can challenge it by
*      initiating a dispute game. If the challenger wins the dispute game
*      the token is not minted and its proposal cancelled.
*
*      The second stage is a claim where the original proposer can receive an actual NFT
*      if it remains unchallenged for sufficiently long.
*
*       ---
*
*      This contract uses on-chain metadata since the metadata itself is required to check the
*      validity of the token. This may be expensive for large metadata.
*/
contract ERC1155ChallengeableMint is ERC1155URIStorage {

    /// @dev Represents a token that has yet to be minted. It must persist
    ///      for at least SETTLEMENT_PERIOD before it can be minted for real
    struct PendingMint {
        /// The address that will receive the token once minted
        address to;
        /// metadata is not stored, only the hash
        bytes32 metadataHash;
        /// hash of public input data to provable computation
        bytes32 witnessHash;
        /// block timestamp when proposal created
        uint256 timestamp;
    }

    /// @dev The amount of time that must pass before a pending mint can be minted
    uint256 public constant SETTLEMENT_PERIOD = 2 hours;

    /// @dev The hash of the code to be run in the provable execution
    Claim public immutable ROOT_CLAIM;

    /// @dev The dispute game factory used to create and track open disputes
    IDisputeGameFactory public immutable disputeGameFactory;
    
    /// @dev Used to generate a fresh token ID for each token
    uint256 public nonce;

    /// @dev Mapping from token nonce to mintable status
    mapping(uint256 => PendingMint) public pendingMints;


    event MintProposed(uint256 indexed id, uint256 timestamp);
    event MintChallenged(uint256 indexed id, address challenger);
    event Minted(uint256 indexed id, address indexed to);

    error MintProposalNotSettled(uint256 id);
    error ProposalHasOpenChallenge(uint256 id);
    error ProposalClaimDisproven(uint256 id);
    error MetadataHashMismatch(uint256 id);
    error CannotChallengeProposalHasSettled(uint256 id);


    /**
    * @dev Construct a new NFTCollection contract.
     */
    constructor(address disputeGameFactoryAddr, Claim rootClaim) ERC1155("") {
        disputeGameFactory = IDisputeGameFactory(disputeGameFactoryAddr);
        ROOT_CLAIM = rootClaim;
        _setBaseURI("data:application/json;base64,");
    }

    /**
    * @dev Propose a new mint.
    * @param metadataJson The string of metadata.json to associate with the token.
    * @param witness The witness given as input to the provable program 
    *                to produce this metadata output.
    *                This must be provided in calldata so watchers can replicate the computation
    */
    function ProposeMint(address to, string calldata metadataJson, bytes calldata witness) external {
        uint256 id = nonce++;
        pendingMints[id] = PendingMint({
            to: to,
            metadataHash: keccak256(bytes(metadataJson)),
            witnessHash: keccak256(witness),
            timestamp: block.timestamp
        });
        emit MintProposed(id,  block.timestamp);
    }

    /**
    * @dev Challenge a pending mint.
    *    This will lead to the creation of a challenge game on this mint.
    *    The winner of the game receives 1/2 the bond from the opponent.
    *    The rest goes into the dev fund :)
    * @param id The ID of the pending mint to challenge
    */
    function ChallengeMint(uint256 id) external {
        PendingMint memory proposal = pendingMints[id];

        if (block.timestamp > proposal.timestamp + SETTLEMENT_PERIOD)
            revert CannotChallengeProposalHasSettled(id);

        disputeGameFactory.create(
            GameTypes.FAULT,
            ROOT_CLAIM,
            bytes(abi.encodePacked(proposal.to, proposal.metadataHash, proposal.witnessHash, proposal.timestamp))
        );

        emit MintChallenged(id, msg.sender);
    }

    /**
    * @dev Mint a new token with the given metadata.
    * @param id The ID of the pending mint (will also be the ID of the token)
    * @param metadataJson The string of metadata.json to associate with the token.
    *                     Must match the hash stored in pending
    */
    function ClaimMint(uint256 id, string memory metadataJson) external {
        PendingMint storage proposal = pendingMints[id];

        if (block.timestamp <= proposal.timestamp + SETTLEMENT_PERIOD)
            revert MintProposalNotSettled(id);

        if (proposal.metadataHash != keccak256(bytes(metadataJson)))
            revert MetadataHashMismatch(id);

        // see if there is an open challenge
        (IDisputeGame game,) = disputeGameFactory.games(
            GameTypes.FAULT,
            ROOT_CLAIM,
            bytes(abi.encodePacked(proposal.to, proposal.metadataHash, proposal.witnessHash, proposal.timestamp))
        );
        if (address(game) != address(0) && game.status() == GameStatus.IN_PROGRESS) {
            revert ProposalHasOpenChallenge(id);
        }
        if (address(game) != address(0) && game.status() == GameStatus.DEFENDER_WINS) {
            revert ProposalClaimDisproven(id);
        }

        _mint(proposal.to, id, 1, "");
        _setURI(id, Base64.encode(bytes(metadataJson)));
        delete pendingMints[id];

        emit Minted(id, proposal.to);
    }

    /**
     * @dev Get the address of the dispute game for the given pending mint
     */
    function gameAddress(uint256 id) public view returns (IDisputeGame game) {
        PendingMint memory proposal = pendingMints[id];
        (game,) = disputeGameFactory.games(
            GameTypes.FAULT,
            ROOT_CLAIM,
            bytes(abi.encodePacked(proposal.to, proposal.metadataHash, proposal.witnessHash, proposal.timestamp))
        );
    }
}
