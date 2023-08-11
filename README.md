# ProofBoy
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

You caught a shiney pokemon? Prove it!

ProofBoy allows for proving state of Gameboy games on-chain using fraud proofs.

Created as an entry to the 2023 Consensys NAV Hackathon.

<!-- ABOUT THE PROJECT -->
## About The Project

Many blockchain games produce assets such as NFTs to reward players for certain in-game achievements. The common way to verify that players actually did the achievements they claim is to have a game server performing these checks and minting the assets.

But we want to move to a world of fully on-chain games! In this case the chain itself must verify the achievements within its restricted execution environment. ProofBoy shows how something as complex as a Gameboy emulator can be proven on-chain using fault proofs.

### How it works

Players play a game off-chain inside a special emulator called the **ProofBoy Recorder**. This works just like a regular emulator but it keeps track of every key press the player makes and the exact CPU cycle they occur at. This record of inputs we call a Journal.

This journal can be taken by anyone else and replayed into their own emulator to recreate the exact same Gameboy memory state. ProofBoy also defines something called an Extractor which takes a memory state and maps it into something meaningful. In our example we have an extractor which extracts the stats of the Pokemon in the first party slot.

Both of these operations are deterministic. Anyone can take a fresh copy of a game and a journal obtain the pokemon the player is holding.

---

To prove this on-chain there is a second piece called the **Proofboy Verifier**. This is another emulator but operates in a headless mode. It is built to compile to MIPS in a way that is compatible with the Optimism Cannon fault proving system. Cannon was built to verify the Optimism optimistic rollup state transaction on L1 but it can actually be used for any MIPS program including our emulator!

Watchers can execute the **Proofboy verifier** off-chain inside the Cannon MIPS emulator (emulators within emulators!). If they identify a fraudulent claim (e.g. you claim your journal produces a Pokemon that it doesn't) they can challenge it on-chain.

The program itself never fully executed on-chain but if there is a fault in a claim then an honest observer can always show that you are committing fraud. If no one can show your claim is fraudulent for sufficiently long it is assumed to be correct. Just like an optimistic rollup. 

---

The example implemented for the hackathon allows players to mint an NFT for their favorite Pokemon. The flow works as follows:

- They play the game inside the Proofboy Recorder until the pokemon they wish to mint is first in their party
- They freeze the state which outputs a journal of all their actions and a JSON metadata object describing the pokemon NFT
- The journal and Metadata are submitted along with a bond to a special ERC1155 contract **ERC1155ChallengeableMint**. This allows players to call `ProposeMint` to propose a mint which must remain unchallenged for 2 hours before it can be claimed

From this point two things can occur:

1. The player was honest so no one can challenge their claim. After 2 hours the mint settles they can call `ClaimMint` to receive their Pokemon NFT and their bond back
2. The player was dishonest. An honest watcher can call `ChallengeMint` to initiate a dispute game. This plays out according to the [Optimism dispute game](https://blog.oplabs.co/building-a-fault-proof-system/) and if the challenge is successful they are awarded the bond and the mint is cancelled

Under the assumption that at least one honest watcher is checking any given mint then mints which are allowed to settle can be considered valid. Any invalid mint should be challenged because it is free money for the challenger. These are the same assumptions under which an optimistic rollup operates however since the value secured is much less the settlement period can be reduced from 7 days to a more acceptable few hours.

## Repo Overview

| Component | Description | Doc |
| -------- | -------- | -------- |
| Contracts     | Contains the ERC1155ChallengeableMint contract along with forks of the Optimism dispute game and VM contracts | [![documentation](https://img.shields.io/badge/readme-blue)](./contracts)  |
| Proofboy Recorder  | A Rust Gameboy emulator which records every key press in a journal. Can be built for desktop or web via WASM | [![documentation](https://img.shields.io/badge/readme-blue)](./crates/proofboy-recorder/)  |
| Proofboy Verifier Cannon  | A headless Gameboy emulator which recreates state given a journal. Builds to a Cannon compatible MIPS binary which reads inputs via a pre-image oracle | [![documentation](https://img.shields.io/badge/readme-blue)](./crates/proofboy-verifier-cannon/)  |
| Preimage Server | A Cannon compatible pre-image server which fetches the required input data to verify a proof from calldata on the given chain | [![documentation](https://img.shields.io/badge/readme-blue)](./crates/preimage-server/)  |
| Client | React web client for ProofBoy. Allows playing games and submitting NFT claims directly from within the browser | [![documentation](https://img.shields.io/badge/readme-blue)](./client/)  |


<!-- GETTING STARTED -->
## Getting Started


### Prerequisites

<!-- USAGE EXAMPLES -->
## Usage


<!-- Makes use of -->

## Makes Possible By

<p align="middle" float="left">
  <a href="https://chainsafe.io/">
    <img src="https://drand.love/images/loe/logo-chainsafe.svg" width="100" style="padding-right: 30px"/>
  </a>
  <text>
  <a href="https://buildwithsygma.com">
  <img src="https://buildwithsygma.com/orange-on-gray.png" width="100" style="padding-right: 30px"/> 
  </a>
</p>

<!-- LICENSE -->
## License

Distributed under the MIT License. See [LICENSE](./LICENSE) for more information.


<!-- CONTACT -->
## Contact

Willem Olding - [@willemolding1](https://twitter.com/willemolding1) - willemolding@gmail.com

Project Link: [https://github.com/willemolding/ProofBoy](https://github.com/willemolding/ProofBoy)

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/willemolding/ProofBoy.svg?style=for-the-badge
[contributors-url]: https://github.com/willemolding/ProofBoy/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/willemolding/ProofBoy.svg?style=for-the-badge
[forks-url]: https://github.com/willemolding/ProofBoy/network/members
[stars-shield]: https://img.shields.io/github/stars/willemolding/ProofBoy.svg?style=for-the-badge
[stars-url]: https://github.com/willemolding/ProofBoy/stargazers
[issues-shield]: https://img.shields.io/github/issues/willemolding/ProofBoy.svg?style=for-the-badge
[issues-url]: https://github.com/willemolding/ProofBoy/issues
[license-shield]: https://img.shields.io/github/license/willemolding/ProofBoy.svg?style=for-the-badge
[license-url]: https://github.com/willemolding/ProofBoy/blob/master/LICENSE.txt
