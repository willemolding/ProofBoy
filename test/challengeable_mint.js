const ERC1155ChallengeableMint = artifacts.require("ERC1155ChallengeableMint");
const {
  BN,           // Big Number support
  constants,    // Common constants, like the zero address and largest integers
  expectEvent,  // Assertions for emitted events
  expectRevert, // Assertions for transactions that should fail
  time,
} = require('@openzeppelin/test-helpers');

contract("ERC1155ChallengeableMint", accounts => {
  let proposer = accounts[1];
  let metadata = "test";

  it("Can propose mint", async () => {
    const contract = await ERC1155ChallengeableMint.deployed();

    await contract.ProposeMint(proposer, metadata, []);

    // nonce was incremented
    assert.equal(await contract.nonce(), 1);

    // there is a pending mint with correct data
    let pending = await contract.pendingMints(0)
    assert.equal(pending.to, proposer);
    assert.equal(pending.metadataHash, web3.utils.keccak256(metadata));

  });

  it("Cannot claim before settlement", async () => {
    const contract = await ERC1155ChallengeableMint.deployed();
    await contract.ProposeMint(proposer, metadata, []);
    await expectRevert.unspecified(contract.ClaimMint(0, metadata))
  });

  it("Can claim after settlement", async () => {
    const contract = await ERC1155ChallengeableMint.deployed();
    await contract.ProposeMint(proposer, metadata, []);
    await time.increase(60*60*60)
    await contract.ClaimMint(0, metadata)
  });

  it("Can open challenge", async () => {
    const contract = await ERC1155ChallengeableMint.deployed();
    await contract.ChallengeMint(0);
  });

});
