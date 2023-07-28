import styles from './NftPreview.module.css'
import { NftMetadata } from '../../types';

import Image from 'react-bootstrap/Image';
import Table from 'react-bootstrap/Table';

export const NftPreview = ({ metadata }: { metadata: NftMetadata }) => {

  if (metadata === undefined) {
    return  <div className={styles.nft_preview}></div>
  }

  return (
    <div className={styles.nft_preview}>
      <Image src={metadata.image} alt={metadata.name} thumbnail />
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
    </div>
  )
}