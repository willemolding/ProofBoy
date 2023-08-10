import './App.global.css'
import styles from './App.module.css'
import 'bootstrap/dist/css/bootstrap.css'

import Stack from 'react-bootstrap/Stack';

import { useState } from 'react';

import { Navigation } from './components/Navigation'
import { Display } from './components/Display'
import { ProofBoyPlayer } from './components/ProofBoyPlayer'
import { PendingMints } from './components/PendingMints'
import { MetaMaskError } from './components/MetaMaskError'
import { MetaMaskContextProvider } from './hooks/useMetaMask'
import { NftPreview } from './components/NftPreview';

import { ProofBoyData, NftMetadata } from './types';

import Tab from 'react-bootstrap/Tab';
import Tabs from 'react-bootstrap/Tabs';
import Container from 'react-bootstrap/Container';

export const App = () => {

  const [proofBoyData, setProofboyData] = useState({} as ProofBoyData);

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
      <div className={styles.appContainer}>
      <Navigation />
      <Container>
        <Tabs
          defaultActiveKey="recorder"
        >
          <Tab eventKey="recorder" title="ProofBoy Recorder">
            <Stack className="d-flex align-items-center justify-content-center text-center min-vh-100">
              {/* <ProofBoyPlayer onNewData={onProofBoyData}/> */}
              <NftPreview proofBoyData={proofBoyData}/>
              <Display/>
            </Stack>
          </Tab>
          <Tab eventKey="claim" title="Claim Mints">
            <PendingMints />
          </Tab>
        </Tabs>
      </Container>
      <MetaMaskError />

      </div>
    </MetaMaskContextProvider>
  )
}