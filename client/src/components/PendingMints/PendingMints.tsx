import { useState, useEffect } from 'react';
import styles from './PendingMints.module.css'
import { useMetaMask } from '~/hooks/useMetaMask'
import { ethers } from "ethers";
import ERC1155ChallengeableMint from '../../contracts/ERC1155ChallengeableMint.json';
import { NftMetadata, ProofBoyData } from '~/types';
import { Button } from 'react-bootstrap';
import Card from 'react-bootstrap/Card';
import Container from 'react-bootstrap/Container';
import Row from 'react-bootstrap/Row';
import {registerNft} from '~/utils';
import Stack from 'react-bootstrap/Stack';
import Table from 'react-bootstrap/Table';

import { gql, useQuery } from '@apollo/client';

export const PendingMints = ({indexedNfts}: {indexedNfts: Map<Number, ProofBoyData>}) => {

  const { wallet } = useMetaMask()

  const GET_PENDING_MINTS = gql`
    query GetPendingMints {
      pendingMints {
        id
        timestamp
        to
        token_id
        calldata
        txn_hash
      }
    }
  `;

  const { loading, error, data } = useQuery(GET_PENDING_MINTS)

  const claimMint = async (id: Number, metadataString: string) => {
    const provider = new ethers.BrowserProvider(window.ethereum);
    const signer = await provider.getSigner();

    // @ts-ignore
    const contractAddress: string = ERC1155ChallengeableMint.networks[wallet.chainId].address;
    let contract = new ethers.Contract(contractAddress, ERC1155ChallengeableMint.abi, signer);

    try {
      await contract.ClaimMint(id, metadataString)
    } catch (error) {
      console.log(error)
    }

    contract.on("Minted", (tokenId, to) => {
      // register with MetaMask SDK so it will appear in the wallet right away :)
      registerNft(window.ethereum, contractAddress, tokenId.toString())
    });

  }
  if(loading) {
    return(
      <div>Loading...</div>
    )
  } else {
    return (
      <div className={styles.nft_preview}>
        <Row>
        {
          data.pendingMints.map(({id, token_id, to, timestamp, calldata, txn_hash}: any) => {

            // decode metadata from calldata
            let iface = new ethers.Interface(ERC1155ChallengeableMint.abi);
            let [too, metadataString, witness] = iface.decodeFunctionData("ProposeMint", calldata);
            let metadata: NftMetadata = JSON.parse(metadataString);

            const timeToClaim = 60*1000 - (Date.now() - Number(timestamp)*1000) // in milliseconds

            // const timeToClaim = 2*60*60*1000 - (Date.now() - Number(timestamp)*1000) // in milliseconds

            return(
              <Card key={id} style={{ width: '20rem' }}>
                <Card.Body>
                  <Card.Title>ID: {token_id}</Card.Title>
                  <Card.Img variant="top" src={metadata.image} />
                  <Table striped bordered hover>
                    <thead>
                      <tr>
                        <th>Stat</th>
                        <th>Value</th>
                      </tr>
                    </thead>
                    <tbody>
                    {
                      metadata.attributes.map(({trait_type, value}: {trait_type: string, value: string}, index: Number) => {
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
                  <Card.Text>
                    Recipient: {to}
                    <br />
                    Tx Hash: {txn_hash}
                  </Card.Text>
                    {timeToClaim > 0 ? <Button variant="danger">Challenge</Button> : <Button variant="success" onClick={() => claimMint(token_id, metadataString)}>Claim</Button>}
                </Card.Body>
                <Card.Footer>
                  <small className="text-muted">{Math.round(timeToClaim / 1000 / 60)} minutes until claimable</small>
                </Card.Footer>
              </Card>
            )
          })
        }
        </Row>
      </div>
    )
  }
}