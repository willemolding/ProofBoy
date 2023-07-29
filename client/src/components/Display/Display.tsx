import { useMetaMask } from '~/hooks/useMetaMask'
import { formatChainAsNum } from '~/utils'
import styles from './Display.module.css'

import NFTCollection from '../../contracts/NFTCollection.json';

import { Button } from 'react-bootstrap';
import { ProofBoyData } from '~/types';
import { registerNft } from '~/utils';

import { ethers } from "ethers";

export const Display = ({ proofBoyData }: { proofBoyData: ProofBoyData }) => {

  const { wallet } = useMetaMask()

  const submitNft = async () => {

    const provider = new ethers.BrowserProvider(window.ethereum);
    // It will prompt user for account connections if it isnt connected
    const signer = await provider.getSigner();
    console.log("Account:", await signer.getAddress());

    // @ts-ignore
    const contractAddress: string = NFTCollection.networks[wallet.chainId].address;
    let contract = new ethers.Contract(contractAddress, NFTCollection.abi, signer);

    await contract.mint(wallet.accounts[0], JSON.stringify(proofBoyData.data))

    contract.on("Minted", (to, tokenId) => {
      registerNft(window.ethereum, contractAddress, tokenId.toString())
    });

  }

  const saveJournal = (journal: Uint8Array | undefined) => {
    if (journal !== undefined) {
      const a = document.createElement('a');
      a.download = 'journal.proofboy';
      a.href = URL.createObjectURL(new Blob([journal]));
      a.addEventListener('click', (e) => {
        setTimeout(() => URL.revokeObjectURL(a.href), 30 * 1000);
      });
      a.click();
    } else {
      console.log("Journal is undefined, cannot download")
    }
  }

  return (
    <div className={styles.display}>
      {wallet.accounts.length > 0 &&
        <>
          <div>Wallet Accounts: {wallet.accounts[0]}</div>
          <div>Wallet Balance: {wallet.balance}</div>
          <div>Hex ChainId: {wallet.chainId}</div>
          <div>Numeric ChainId: {formatChainAsNum(wallet.chainId)}</div>
        </>
      }
      <Button onClick={submitNft}>Submit</Button>
      <Button onClick={() => saveJournal(proofBoyData.journal)}>Save Journal</Button>
    </div>
  )
}