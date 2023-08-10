import { useState, useEffect } from 'react';
import styles from './PendingMints.module.css'
import { NftMetadata } from '../../types';
import { useMetaMask } from '~/hooks/useMetaMask'
import { ethers } from "ethers";
import ERC1155ChallengeableMint from '../../contracts/ERC1155ChallengeableMint.json';
import { ProofBoyData } from '~/types';
import { Button } from 'react-bootstrap';
import Card from 'react-bootstrap/Card';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import {registerNft} from '~/utils';
import Stack from 'react-bootstrap/Stack';

export const PendingMints = ({indexedNfts}: {indexedNfts: Map<Number, ProofBoyData>}) => {

  const { wallet } = useMetaMask()

  const [pendingMints, setPendingMints] = useState<Array<any>>([]);

  useEffect(() => {
    const retrievePending = async () => {
      const provider = new ethers.BrowserProvider(window.ethereum);
      const network = await provider.getNetwork();
      // @ts-ignore
      const contractAddress: string = ERC1155ChallengeableMint.networks[network.chainId].address;
      let contract = new ethers.Contract(contractAddress, ERC1155ChallengeableMint.abi, provider);

      let retrieved = [];
      for (const [id, value] of indexedNfts.entries()) {
        let [to, metadataHash, witnessHash, timestamp] = await contract.pendingMints(id);
        if (to !== ethers.ZeroAddress) {
          retrieved.push({
            id,
            to: to,
            metadata: value.data,
            journal: value.journal,
            timestamp: timestamp
          });
        }
      }
      setPendingMints(retrieved)
    }
    retrievePending();
  }, [wallet, indexedNfts]);

  const claimMint = async (id: Number) => {
    const provider = new ethers.BrowserProvider(window.ethereum);
    const signer = await provider.getSigner();

    // @ts-ignore
    const contractAddress: string = ERC1155ChallengeableMint.networks[wallet.chainId].address;
    let contract = new ethers.Contract(contractAddress, ERC1155ChallengeableMint.abi, signer);

    try {
      await contract.ClaimMint(id, JSON.stringify(indexedNfts.get(id)?.data))
    } catch (error) {
      console.log(error)
    }

    contract.on("Minted", (tokenId, to) => {
      registerNft(window.ethereum, contractAddress, tokenId.toString())
    });

  }

  return (
    <div className={styles.nft_preview}>
      <Row>
      {
        pendingMints.map(({id, to, metadataHash, witnessHash, timestamp}, index) => {
          return(
            <Card key={index} style={{ width: '20rem' }}>
              <Card.Body>
                <Card.Title>ID: {id}</Card.Title>
                <Card.Img variant="top" src="https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/0.png" />
                <Card.Text>
                  <p>Submitted By: {to}</p>
                </Card.Text>
                <Stack direction="horizontal" gap={3}>
                  <Button onClick={() => claimMint(id)}>Claim</Button>
                  <Button>Challenge</Button>
                </Stack>
              </Card.Body>
              <Card.Footer>
                <small className="text-muted">Submitted {Math.round((Date.now() - Number(timestamp)*1000) / 1000 / 60)} minutes ago</small>
              </Card.Footer>
            </Card>
          )
        })
      }
      </Row>
    </div>
  )
}