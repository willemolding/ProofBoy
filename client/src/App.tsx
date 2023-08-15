import './App.global.css'
import styles from './App.module.css'
import 'bootstrap/dist/css/bootstrap.css'

import Stack from 'react-bootstrap/Stack';

import { useState } from 'react';
import { ApolloProvider, ApolloClient, InMemoryCache, gql } from '@apollo/client';

import { Navigation } from './components/Navigation'
import { Display } from './components/Display'
import { ProofBoyPlayer } from './components/ProofBoyPlayer'
import { PendingMints } from './components/PendingMints'
import { MetaMaskError } from './components/MetaMaskError'
import { MetaMaskContextProvider } from './hooks/useMetaMask'
import { NftPreview } from './components/NftPreview';

import { ProofBoyData, } from './types';

import Tab from 'react-bootstrap/Tab';
import Tabs from 'react-bootstrap/Tabs';
import Container from 'react-bootstrap/Container';

const GET_MY_MINTS = gql`
query GetPendingMints($to: Bytes!) {
  pendingMints(where: {to: $to, disputed: false}) {
    id
    timestamp
    to
    token_id
    calldata
    txn_hash
  }
}
`;

const GET_CHALLENGABLE_MINTS = gql`
query GetPendingMints($to: Bytes!) {
  pendingMints(where: {to_not: $to, disputed: false}) {
    id
    timestamp
    to
    token_id
    calldata
    txn_hash
  }
}
`;

export const App = () => {

  const graphClient = new ApolloClient({
    cache: new InMemoryCache(),
    uri: "http://localhost:8000/subgraphs/name/willemolding/ProofBoy"
  });

  const [proofBoyData, setProofboyData] = useState({} as ProofBoyData);
  const [indexedNfts, setIndexedNfts] = useState<Map<Number, ProofBoyData>>(new Map());

  const addIndexedNft = (k: Number, v: ProofBoyData) => {
    setIndexedNfts(new Map(indexedNfts.set(k, v)));
  };

  const onProofBoyData = (data: string, journal: string) => {
    console.log(data)
    console.log(journal)
    setProofboyData({
      data: JSON.parse(data),
      journal: Uint8Array.from(JSON.parse(journal))
    })
  }

  return (
    <MetaMaskContextProvider>
    <ApolloProvider client={graphClient}>
      <div className={styles.appContainer}>
      <Navigation />
      <Container>
        <Tabs
          defaultActiveKey="recorder"
        >
          <Tab eventKey="recorder" title="ProofBoy Recorder">
            <Stack className="d-flex align-items-center justify-content-center text-center min-vh-100">
              <ProofBoyPlayer onNewData={onProofBoyData}/>
              <NftPreview proofBoyData={proofBoyData} addIndexedNft={addIndexedNft}/>
              <Display/>
            </Stack>
          </Tab>
          <Tab eventKey="claim" title="My Claims">
            <PendingMints query={GET_MY_MINTS} claim/>
          </Tab>
          <Tab eventKey="challenge" title="Challengeable">
            <PendingMints query={GET_CHALLENGABLE_MINTS} challenge/>
          </Tab>
        </Tabs>
      </Container>
      <MetaMaskError />

      </div>
    </ApolloProvider>
    </MetaMaskContextProvider>
  )
}
