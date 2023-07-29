var NFTCollection = artifacts.require("./NFTCollection.sol");

module.exports = function(deployer) {
  deployer.deploy(NFTCollection);
};
