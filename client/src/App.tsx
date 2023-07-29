import './App.global.css'
import styles from './App.module.css'
import 'bootstrap/dist/css/bootstrap.css'

import { useState } from 'react';

import { Navigation } from './components/Navigation'
import { Display } from './components/Display'
import { ProofBoyPlayer } from './components/ProofBoyPlayer'
import { MetaMaskError } from './components/MetaMaskError'
import { MetaMaskContextProvider } from './hooks/useMetaMask'
import { NftPreview } from './components/NftPreview';

import { ProofBoyData, NftMetadata } from './types';

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
        <ProofBoyPlayer onNewData={onProofBoyData} startupJournal={proofBoyData.journal}/>
        <NftPreview metadata={proofBoyData.data}/>
        <Display proofBoyData={proofBoyData} onJournalUpload={ (j) => { console.log("new journal", j); setProofboyData({journal: j} as ProofBoyData) } }/>
        <MetaMaskError />
      </div>
    </MetaMaskContextProvider>
  )
}