//SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import { MIPS } from "@eth-optimism/contracts-bedrock/src/cannon/MIPS.sol";
import { PreimageOracle } from "@eth-optimism/contracts-bedrock/src/cannon/PreimageOracle.sol";

contract VM is MIPS {
    constructor() {
        oracle = new PreimageOracle();
    }
}
