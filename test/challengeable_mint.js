const ERC1155ChallengeableMint = artifacts.require("ERC1155ChallengeableMint");
const FaultDisputeGame = artifacts.require("FaultDisputeGame");

const {
  BN,           // Big Number support
  constants,    // Common constants, like the zero address and largest integers
  expectEvent,  // Assertions for emitted events
  expectRevert, // Assertions for transactions that should fail
  time,
} = require('@openzeppelin/test-helpers');

contract("ERC1155ChallengeableMint proposals", accounts => {
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
    await expectRevert.unspecified(contract.ClaimMint(0, metadata))
  });

  it("Can claim after settlement", async () => {
    const contract = await ERC1155ChallengeableMint.deployed();
    await time.increase(60*60*60)
    await contract.ClaimMint(0, metadata)
  });
});

contract("ERC1155ChallengeableMint challenges", accounts => {
  let proposer = accounts[1];
  let challenger = accounts[2];
  let disputeGameAddr;

  let metadata = "test";

  it("Can open challenge", async () => {
    const contract = await ERC1155ChallengeableMint.deployed();
    await contract.ProposeMint(proposer, metadata, []);
    assert.equal(await contract.nonce(), 1);
    // simulate the call to obtain what the contract address will be
    disputeGameAddr = await contract.ChallengeMint.call(0, { from: challenger });
    await contract.ChallengeMint(0, { from: challenger });
  });

  it("Cannot open multiple challenges", async () => {
    const contract = await ERC1155ChallengeableMint.deployed();
    await expectRevert.unspecified(contract.ChallengeMint(0, { from: challenger }));
  });

  it("Cannot claim if challenge open", async () => {
    const contract = await ERC1155ChallengeableMint.deployed();
    await time.increase(60*60*60)
    await expectRevert.unspecified(contract.ClaimMint(0, metadata))
  });

  it("Challenge can be cleared by timeout and mint claimed", async () => {
    const contract = await ERC1155ChallengeableMint.deployed();
    let faultGame = await FaultDisputeGame.at(disputeGameAddr);
    await time.increase(86400)
    await faultGame.resolve()
    await contract.ClaimMint(0, metadata)
  })
});
