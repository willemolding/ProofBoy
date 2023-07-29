import styles from './ProofBoyPlayer.module.css'
import { useEffect } from 'react'

import init, { run } from '../../../../crates/proofboy-recorder/pkg/proofboy_recorder'

export const ProofBoyPlayer = ({ onNewData, startupJournal }: {onNewData: (data: string, journal: string) => void, startupJournal?: Uint8Array}) => {
  useEffect(() => {
    const runBevyApp = async () => {
      await init()
      run('#proof-boy-canvas', onNewData, startupJournal)
    }
    runBevyApp()
  }, [])

  return (
    <div className={styles.proofBoyPlayer}>
      <canvas id="proof-boy-canvas"></canvas>
    </div>
  )
}
