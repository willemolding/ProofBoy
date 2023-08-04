//SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import { FaultDisputeGame as OpFaultDisputeGame } from "@eth-optimism/contracts-bedrock/src/dispute/FaultDisputeGame.sol";
import { IBigStepper } from "@eth-optimism/contracts-bedrock/src/dispute/interfaces/IBigStepper.sol";
import { L2OutputOracle } from "@eth-optimism/contracts-bedrock/src/L1/L2OutputOracle.sol";
import { BlockOracle } from "@eth-optimism/contracts-bedrock/src/dispute/BlockOracle.sol";
import { Claim, Duration } from "@eth-optimism/contracts-bedrock/src/libraries/DisputeTypes.sol";

contract FaultDisputeGame is OpFaultDisputeGame {
    constructor(
        Claim _absolutePrestate,
        uint256 _maxGameDepth,
        Duration _gameDuration,
        IBigStepper _vm
    ) OpFaultDisputeGame(_absolutePrestate, _maxGameDepth, _gameDuration, _vm, L2OutputOracle(address(0)), BlockOracle(address(0))) {}
}
