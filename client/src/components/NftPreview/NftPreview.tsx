import styles from './NftPreview.module.css'
import { NftMetadata } from '../../types';
import { useMetaMask } from '~/hooks/useMetaMask'
import { ethers } from "ethers";
import ERC1155ChallengeableMint from '../../contracts/ERC1155ChallengeableMint.json';
import { ProofBoyData } from '~/types';
import { Button } from 'react-bootstrap';

import Image from 'react-bootstrap/Image';
import Table from 'react-bootstrap/Table';

export const NftPreview = ({ proofBoyData, addIndexedNft }: { proofBoyData: ProofBoyData, addIndexedNft: any}) => {

  const { wallet } = useMetaMask()

  const submitMintProposal = async () => {
    const provider = new ethers.BrowserProvider(window.ethereum);
    const signer = await provider.getSigner();

    // @ts-ignore
    const contractAddress: string = ERC1155ChallengeableMint.networks[wallet.chainId].address;
    let contract = new ethers.Contract(contractAddress, ERC1155ChallengeableMint.abi, signer);
    await contract.ProposeMint(wallet.accounts[0], JSON.stringify(proofBoyData.data), proofBoyData.journal)

    contract.on("MintProposed", (id, timestamp) => {
      addIndexedNft(Number(id), proofBoyData)
    })
  }

  const metadata = proofBoyData.data as NftMetadata;

  if (metadata === undefined) {
    return  <div className={styles.nft_preview}></div>
  }

  return (
    <div className={styles.nft_preview}>
      <Image src={metadata.image} alt={metadata.name} width={"100px"} thumbnail />
      <h3>{metadata.name}</h3>
      <Table striped bordered hover>
      <thead>
        <tr>
          <th>Stat</th>
          <th>Value</th>
        </tr>
      </thead>
      <tbody>
      {
        metadata.attributes.map(({trait_type, value}, index) => {
          return(
            <tr>
              <td>{trait_type}</td>
              <td>{value}</td>
            </tr>
          )
        })
      }
      </tbody>
      </Table>
      <Button onClick={submitMintProposal}>Submit</Button>
    </div>
  )
}