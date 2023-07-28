import styles from './ProofBoyPlayer.module.css'
import { useEffect } from 'react'

import init, { run } from '../../../../proofboy-recorder/pkg'

export const ProofBoyPlayer = ({ onNewData }: {onNewData: (data: string, journal: string) => void}) => {
  useEffect(() => {
    const runBevyApp = async () => {
      await init()
      run('#proof-boy-canvas', onNewData)
    }
    runBevyApp()
  }, [])

  return (
    <div className={styles.proofBoyPlayer}>
      <canvas id="proof-boy-canvas"></canvas>
    </div>
  )
}
