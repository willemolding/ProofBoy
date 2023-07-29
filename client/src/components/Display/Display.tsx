import { useMetaMask } from '~/hooks/useMetaMask'
import { formatChainAsNum } from '~/utils'
import styles from './Display.module.css'

import NFTCollection from '../../contracts/NFTCollection.json';

import { Button, Form } from 'react-bootstrap';
import { ProofBoyData } from '~/types';
import { registerNft } from '~/utils';

import { ethers } from "ethers";

export const Display = ({ proofBoyData, onJournalUpload }: { proofBoyData: ProofBoyData, onJournalUpload?: (j: Uint8Array) => void }) => {

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

  const loadJournal = (e: React.ChangeEvent<HTMLInputElement>) => {
    // @ts-ignore
    const file = e.target?.files[0]
    const reader = new FileReader()
    reader.onload = function(event: ProgressEvent<FileReader>) {
      // got a new file
      if (event.target?.result != null && onJournalUpload != null) {
        onJournalUpload(event.target?.result as Uint8Array)
      }
    }
  
    reader.readAsArrayBuffer(file)
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
      <Form.Group controlId="formFileLg" className="mb-3">
        <Form.Label>Large file input example</Form.Label>
        <Form.Control type="file" accept=".proofboy" size="lg" onChange={loadJournal} />
      </Form.Group>
    </div>
  )
}