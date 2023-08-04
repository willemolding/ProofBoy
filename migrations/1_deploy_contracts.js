var VM = artifacts.require("VM");
var ERC1155ChallengeableMint = artifacts.require("ERC1155ChallengeableMint");

var FaultDisputeGame = artifacts.require("FaultDisputeGame");
var DisputeGameFactory = artifacts.require("DisputeGameFactory");

const ROOT_CLAIM = "0x0";
const MAX_DEPTH = 999999;
const GAME_DURATION = 999999;

module.exports = async function(deployer) {

  await deployer.deploy(VM);
  const VmInstance = await VM.deployed();

  await deployer.deploy(FaultDisputeGame, ROOT_CLAIM, MAX_DEPTH, GAME_DURATION, VmInstance.address);
  const faultDisputeGameInstance = await FaultDisputeGame.deployed();

  await deployer.deploy(DisputeGameFactory);
  const disputeGameFactoryInstance = await DisputeGameFactory.deployed();

  // set the FAULT dispute type to the FaultDisputeGame contract we just deployed
  await disputeGameFactoryInstance.setImplementation(0, faultDisputeGameInstance.address);

  await deployer.deploy(ERC1155ChallengeableMint, disputeGameFactoryInstance.address, ROOT_CLAIM);
};
