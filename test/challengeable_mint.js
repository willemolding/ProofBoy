const ERC1155ChallengeableMint = artifacts.require("ERC1155ChallengeableMint");
const FaultDisputeGame = artifacts.require("FaultDisputeGame");

const {
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
    await time.increase(time.duration.hours(2)+1)
    await contract.ClaimMint(0, metadata)
  });
});

contract("ERC1155ChallengeableMint proposer clear challenge by timeout", accounts => {
  let proposer = accounts[1];
  let challenger = accounts[2];

  let metadata = "test";

  it("Can open challenge", async () => {
    const contract = await ERC1155ChallengeableMint.deployed();
    await contract.ProposeMint(proposer, metadata, []);
    assert.equal(await contract.nonce(), 1);
    await contract.ChallengeMint(0, { from: challenger });
  });

  it("Cannot open multiple challenges", async () => {
    const contract = await ERC1155ChallengeableMint.deployed();
    await expectRevert.unspecified(contract.ChallengeMint(0, { from: challenger }));
  });

  it("Cannot claim if challenge open", async () => {
    const contract = await ERC1155ChallengeableMint.deployed();
    await time.increase(time.duration.hours(2)+1)
    await expectRevert.unspecified(contract.ClaimMint(0, metadata))
  });

  it("Challenge can be cleared by timeout and mint claimed", async () => {
    const contract = await ERC1155ChallengeableMint.deployed();
    let disputeGameAddr = await contract.gameAddress(0);
    let faultGame = await FaultDisputeGame.at(disputeGameAddr);
    await faultGame.resolve()
    await contract.ClaimMint(0, metadata)
  })
});
