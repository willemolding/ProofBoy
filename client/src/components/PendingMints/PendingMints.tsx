import { useState } from 'react';
import styles from './PendingMints.module.css'
import { NftMetadata } from '../../types';
import { useMetaMask } from '~/hooks/useMetaMask'
import { ethers } from "ethers";
import ERC1155ChallengeableMint from '../../contracts/ERC1155ChallengeableMint.json';
import { ProofBoyData } from '~/types';
import { Button } from 'react-bootstrap';

export const PendingMints = () => {

  const { wallet } = useMetaMask()

  const [pendingMints, setPendingMints] = useState(async () => {
    const provider = new ethers.BrowserProvider(window.ethereum);
    // @ts-ignore
    const contractAddress: string = ERC1155ChallengeableMint.networks[wallet.chainId].address;
    let contract = new ethers.Contract(contractAddress, ERC1155ChallengeableMint.abi);
    let pendingMints = [];
    for (let i = 0; i < 10; i++) {
      let pendingMint = await contract.pendingMints(i);
      if (pendingMint.timestamp !== 0) {
        pendingMints.push(pendingMint);
      }
    }
    return pendingMints
  });


  // const claimMint = async () => {
  //   const provider = new ethers.BrowserProvider(window.ethereum);
  //   const signer = await provider.getSigner();
  //   console.log("Account:", await signer.getAddress());

  //   // @ts-ignore
  //   const contractAddress: string = ERC1155ChallengeableMint.networks[wallet.chainId].address;
  //   let contract = new ethers.Contract(contractAddress, ERC1155ChallengeableMint.abi, signer);
  //   // TODO: Fix hardcoded claim ID
  //   await contract.ClaimMint(0, JSON.stringify(proofBoyData.data))

  //   contract.on("Minted", (tokenId, to) => {
  //     registerNft(window.ethereum, contractAddress, tokenId.toString())
  //   });

  // }

  return (
    <div className={styles.nft_preview}>

    </div>
  )
}