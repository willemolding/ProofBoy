import styles from './ProofBoyPlayer.module.css'
import { useEffect } from 'react'

import init, { run } from "../../../../proofboy-recorder/pkg";

export const ProofBoyPlayer = () => {

useEffect(() => {
    const runBevyApp = async () => {
        await init()
        run("#proof-boy-canvas", (data: string) => {
            console.log("callback called with: ", data)
        })
    };
    runBevyApp()
    }, [])

  return (
    <div className={styles.proofBoyPlayer}>
        <canvas id="proof-boy-canvas"></canvas>
    </div>
  )
}
