var VM = artifacts.require("VM");
var ERC1155ChallengeableMint = artifacts.require("ERC1155ChallengeableMint");

var FaultDisputeGame = artifacts.require("FaultDisputeGame");
var DisputeGameFactory = artifacts.require("DisputeGameFactory");

const ABSOLUTE_PRESTATE = "0x0"; // set this once the final game binary has been built
const MAX_DEPTH = 999999; // same for this once max game length is known
const GAME_DURATION = 60*60; // 1 hour

module.exports = async function(deployer) {

  await deployer.deploy(VM);
  const VmInstance = await VM.deployed();

  await deployer.deploy(FaultDisputeGame, ABSOLUTE_PRESTATE, MAX_DEPTH, GAME_DURATION, VmInstance.address);
  const faultDisputeGameInstance = await FaultDisputeGame.deployed();

  await deployer.deploy(DisputeGameFactory);
  const disputeGameFactoryInstance = await DisputeGameFactory.deployed();

  // set the FAULT dispute type to the FaultDisputeGame contract we just deployed
  await disputeGameFactoryInstance.setImplementation(0, faultDisputeGameInstance.address);

  await deployer.deploy(ERC1155ChallengeableMint, disputeGameFactoryInstance.address, ABSOLUTE_PRESTATE);
};
