import { useMetaMask } from '~/hooks/useMetaMask'
import { formatChainAsNum } from '~/utils'
import styles from './Display.module.css'

import NFTCollection from '../../contracts/NFTCollection.json';

import { Button } from 'react-bootstrap';
import { NftMetadata } from '~/types';
import { registerNft } from '~/utils';

import { ethers } from "ethers";

export const Display = ({ metadata }: { metadata: NftMetadata }) => {

  const { wallet } = useMetaMask()


  const submitNft = async () => {

    const provider = new ethers.BrowserProvider(window.ethereum);
    // It will prompt user for account connections if it isnt connected
    const signer = await provider.getSigner();
    console.log("Account:", await signer.getAddress());

    // @ts-ignore
    const contractAddress: string = NFTCollection.networks[wallet.chainId].address;
    let contract = new ethers.Contract(contractAddress, NFTCollection.abi, signer);

    await contract.mint(wallet.accounts[0], JSON.stringify(metadata))

    contract.on("Minted", (to, tokenId) => {
      registerNft(window.ethereum, contractAddress, tokenId.toString())
    });

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
    </div>
  )
}