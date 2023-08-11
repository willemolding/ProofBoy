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

Players play a game off-chain inside a special emulator called the **ProofBoy Recorder**. This works just like a regular emulator but it keeps track of every key press made by the player and the exact CPU cycle at which it occurred. This record of inputs we call a Journal.

<p align="center">
<img src="./docs/recorder.excalidraw.svg" width="700"/>
</p>
This journal can be taken by anyone else and replayed into their own emulator to recreate the exact same Gameboy memory state. ProofBoy also defines something called an Extractor which takes a memory state and maps it into something meaningful. In our example we have an extractor which extracts the stats of the Pokemon in the first party slot.

Both of these operations are deterministic. Anyone can take a fresh copy of a game and a journal obtain the pokemon the player is holding.

---

To prove this on-chain there is a second piece called the **Proofboy Verifier**. This is another emulator but operates in a headless mode. It is built to compile to MIPS in a way that is compatible with the Optimism Cannon fault proving system. Cannon was built to verify the Optimism optimistic rollup state transaction on L1 but it can actually be used for any MIPS program including our emulator!

<p align="center">
<img src="./docs/verifier.excalidraw.svg" width="700"/>
</p>

Watchers can execute the **Proofboy verifier** off-chain inside the Cannon MIPS emulator (emulators within emulators!). If they identify a fraudulent claim (e.g. you claim your journal produces a Pokemon that it doesn't) they can challenge it on-chain.

The program itself never fully executed on-chain but if there is a fault in a claim then an honest observer can always show that you are committing fraud. If no one can show your claim is fraudulent for sufficiently long it is assumed to be correct. Just like an optimistic rollup. 

---

For the hackathon we implemented a system that allows players to mint an NFT for their favorite Pokemon. The flow works as follows:

- They play the game inside the Proofboy Recorder until the pokemon they wish to mint is first in their party
- They freeze the state which outputs a journal of all their actions and a JSON metadata object describing the pokemon NFT
- The journal and Metadata are submitted along with a bond to a special ERC1155 contract **ERC1155ChallengeableMint**. This allows players to call `ProposeMint` to propose a mint which must remain unchallenged for 2 hours before it can be claimed

From this point two things can occur:

1. The player was honest so no one can challenge their claim. After 2 hours the mint settles they can call `ClaimMint` to receive their Pokemon NFT and their bond back. This uses the MetaMask SDK to automatically register the minted NFT in the players wallet.

<p align="center">
<img src="./docs/flow-no-challenge.excalidraw.svg" width="700"/>
</p>

2. The player was dishonest. An honest watcher can call `ChallengeMint` to initiate a dispute game. This plays out according to the [Optimism dispute game](https://blog.oplabs.co/building-a-fault-proof-system/) and if the challenge is successful they are awarded the bond and the mint is cancelled

<p align="center">
<img src="./docs/flow-fraud.excalidraw.svg" width="700"/>
</p>

Under the assumption that at least one honest watcher is checking any given mint then mints which are allowed to settle can be considered valid. Any invalid mint should be challenged because it is free money for the challenger. These are the same assumptions under which an optimistic rollup operates however since the value secured is much less the settlement period can be reduced from 7 days to a more acceptable few hours.

## Technologies Used

ProofBoy uses:

- [MetaMask SDK](https://metamask.io/sdk/) to obtain account information and allow the player to submit proposals and claims from the browser. It also uses the ['wallet_WatchAsset'](https://docs.metamask.io/wallet/how-to/display/tokens/) feature to allow the minted NFTs to display in the players wallet right away.
- [Infura](https://www.infura.io/) to allow the Rust preimage-server to retrieve the journal and metadata from a proposal transactions to provide to the verifier
- [Truffle](https://trufflesuite.com/) as the smart contract development suite
- [Ganache](https://trufflesuite.com/docs/ganache/) for local testing

Also a special thanks to:

<p align="middle" float="left">
  <a href="https://metamask.io">
    <img src="https://icodrops.com/wp-content/uploads/2021/01/MetaMask_logo.jpg" width="100" style="padding-right: 30px"/>
  </a>
  <text>
  <a href="https://www.infura.io/">
    <img src="https://images.saasworthy.com/tr:w-178,h-0/infura_8245_logo_1652946244_ozfda.jpg" width="100" style="padding-right: 30px"/>
  </a>
 <text>
  <a href="https://linea.build/">
    <img src="https://images.ctfassets.net/64upluvbiuck/3jYGu3XwBgiRxNPRbzEoyh/53cbb5c2fb09ac12bc073cd15385f625/logo-icon.svg" width="100" style="padding-right: 30px"/>
  </a>
  <text>
  <a href="https://bevyengine.org/">
    <img src="https://bevyengine.org/assets/bevy_logo_dark.svg" width="100" style="padding-right: 30px"/>
  </a>
  <text>
  <a href="https://github.com/BadBoiLabs/Cannon-rs">
    <img src="https://github.com/BadBoiLabs/Cannon-rs/blob/main/resources/cannon-rs-logo.png?raw=true" width="100" style="padding-right: 30px"/>
  </a>
  <text>
  <a href="https://www.openzeppelin.com/">
    <img src="https://cryptocurrencyjobs.co/startups/assets/logos/openzeppelin.8f9eaa9b5b7ba8d557a9053dcb60940fef5b9615e632a3b2e0c35f54db7defae.jpg" width="100" style="padding-right: 30px"/>
  </a>
</p>


## Repo Overview

| Component | Description | Doc |
| -------- | -------- | -------- |
| Contracts     | Contains the ERC1155ChallengeableMint contract along with forks of the Optimism dispute game and VM contracts | [![documentation](https://img.shields.io/badge/readme-blue)](./contracts)  |
| Proofboy Recorder  | A Rust Gameboy emulator which records every key press in a journal. Can be built for desktop or web via WASM. Uses Bevy game engine for Rust | [![documentation](https://img.shields.io/badge/readme-blue)](./crates/proofboy-recorder/)  |
| Proofboy Verifier Cannon  | A headless Gameboy emulator which recreates state given a journal. Builds to a Cannon compatible MIPS binary which reads inputs via a pre-image oracle | [![documentation](https://img.shields.io/badge/readme-blue)](./crates/proofboy-verifier-cannon/)  |
| Preimage Server | A Cannon compatible pre-image server which fetches the required input data to verify a proof from calldata on the given chain using Infura | [![documentation](https://img.shields.io/badge/readme-blue)](./crates/preimage-server/)  |
| Client | React web client for ProofBoy. Allows playing games and submitting NFT claims directly from within the browser | [![documentation](https://img.shields.io/badge/readme-blue)](./client/)  |


<!-- GETTING STARTED -->
## Getting Started



### Prerequisites

#### Rustup

Ensure you have [Rustup](https://rustup.rs/) installed so the specific version of Rust required by ProofBoy can be automatically installed.

#### Docker

If you plan to build the Proofboy Verifier MIPS binary this will require [Docker](https://www.docker.com/) is installed and running

#### Just

This repo uses `just` instead of `make` for build tasks. Install it with

```shell
cargo install just
```

##### Wasm Pack

```shell
cargo install wasm-pack
```

##### Node

A recent version of Node and NPM is required for building the contracts and client. Tested with v20.

<!-- USAGE EXAMPLES -->
## Usage

The following steps can be used to set up a local environment similar to the demo

### 1. Build the ProofBoy Recorder WASM module

```shell
just build_web
```

### 2. Build the contracts

```shell
npm install
npm run build_contracts
```

### 3. Build and run the client

```shell
cd client
npm install
npm run build
npm run preview
```

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
