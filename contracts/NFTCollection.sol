//SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import { Base64 } from "@openzeppelin/contracts/utils/Base64.sol";
import { ERC1155URIStorage, ERC1155 } from "@openzeppelin/contracts/token/ERC1155/extensions/ERC1155URIStorage.sol";
import { IDisputeGameFactory } from "@eth-optimism/contracts-bedrock/src/dispute/interfaces/IDisputeGameFactory.sol";

contract NFTCollection is ERC1155URIStorage {

    /// @dev Represents a token that has yet to be minted. It must persist
    ///      for at least SETTLEMENT_PERIOD before it can be minted for real
    struct PendingMint {
        /// metadata is not stored, only the hash
        bytes32 metadataHash;
        /// journal hash
        bytes32 journalHash;
        /// block timestamp when proposal created
        uint256 timestamp;
    }

    /// @dev The amount of time that must pass before a pending mint can be minted
    uint256 public constant SETTLEMENT_PERIOD = 1 hours;

    /// @dev Used to generate a fresh token ID for each token
    uint256 private nonce;

    /// @dev Mapping from token nonce to mintable status
    mapping(uint256 => PendingMint) public pendingMints;

    /// @dev The dispute game factory used to create and track open disputes
    IDisputeGameFactory public disputeGameFactory;

    event MintProposed(uint256 indexed id, uint256 timestamp);
    event Minted(uint256 indexed id, address indexed to);

    error MintProposalNotSettled(uint256 id);
    error MetadataHashMismatch(uint256 id);

    /**
    * @dev Construct a new NFTCollection contract.
     */
    constructor(address disputeGameFactoryAddr) ERC1155("") {
        disputeGameFactory = IDisputeGameFactory(disputeGameFactoryAddr);
        _setBaseURI("data:application/json;base64,");
    }

    /**
    * @dev Propose a new mint.
    * @param metadataJson The string of metadata.json to associate with the token.
    * @param journal The journal given as input to the provable program 
    *                to produce this metadata output.
    *                This must be provided in calldata so watchers can replicate the computation
    */
    function ProposeMint(string calldata metadataJson, bytes calldata journal) external {
        uint256 id = nonce++;
        pendingMints[id] = PendingMint({
            metadataHash: keccak256(bytes(metadataJson)),
            journalHash: keccak256(journal),
            timestamp: block.timestamp
        });
        emit MintProposed(id,  block.timestamp);
    }

    /**
    * @dev Mint a new token with the given metadata.
    * @param id The ID of the pending mint (will also be the ID of the token)
    * @param metadataJson The string of metadata.json to associate with the token.
    *                     Must match the hash stored in pending
    */
    function mint(address to, uint256 id, string memory metadataJson) external {
        PendingMint storage proposal = pendingMints[id];

        if (proposal.timestamp + SETTLEMENT_PERIOD <= block.timestamp)
            revert MintProposalNotSettled(id);

        if (proposal.metadataHash != keccak256(bytes(metadataJson)))
            revert MetadataHashMismatch(id);

        // TODO: Require no challenges are open

        _mint(to, id, 1, "");
        _setURI(id, Base64.encode(bytes(metadataJson)));

        emit Minted(id, to);
    }
}
