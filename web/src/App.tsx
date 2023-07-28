import './App.global.css'
import styles from './App.module.css'
import 'bootstrap/dist/css/bootstrap.css'

import { createContext } from 'react';

import { Navigation } from './components/Navigation'
import { Display } from './components/Display'
import { ProofBoyPlayer } from './components/ProofBoyPlayer'
import { MetaMaskError } from './components/MetaMaskError'
import { MetaMaskContextProvider } from './hooks/useMetaMask'

export const App = () => {

  const onProofBoyData = (data: string, journal: string) => {
    console.log("Data: ", data)
    console.log("Journal: ", journal)
  }

  return (
    <MetaMaskContextProvider>
      <div className={styles.appContainer}>
        <Navigation />
        <ProofBoyPlayer onNewData={onProofBoyData}/>
        <Display />
        <MetaMaskError />
      </div>
    </MetaMaskContextProvider>
  )
}