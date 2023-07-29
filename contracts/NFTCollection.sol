//SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/utils/Base64.sol";
import "@openzeppelin/contracts/token/ERC1155/extensions/ERC1155URIStorage.sol";

contract NFTCollection is ERC1155URIStorage {

    /// Used to generate a fresh token ID for each token
    uint256 private nonce;

    /**
    * @dev Construct a new NFTCollection contract.
     */
    constructor() ERC1155("") {
        _setBaseURI("data:application/json;base64,");
    }

    /**
    * @dev Mint a new token with the given metadata.
    * @param to The account to give the newly minted token to.
    * @param metadataJson The string of metadata.json to associate with the token.
     */
    function mint(address to, string memory metadataJson) public {
        uint256 id = nonce++;
        _mint(to, id, 1, "");
        _setURI(id, Base64.encode(bytes(metadataJson)));

        emit Minted(to, id);
    }

    event Minted(address indexed to, uint256 indexed id);
}
