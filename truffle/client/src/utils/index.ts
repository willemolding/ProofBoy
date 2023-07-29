export const formatBalance = (rawBalance: string) => {
  const balance = (parseInt(rawBalance) / 1000000000000000000).toFixed(2)
  return balance
}

export const formatChainAsNum = (chainIdHex: string) => {
  const chainIdNum = parseInt(chainIdHex)
  return chainIdNum
}

export const formatAddress = (addr: string) => {
  return `${addr.substring(0, 8)}...`
}

export const registerNft = async (ethereum: any, address: string, tokenId: string) => {
  // 'wasAdded' is a boolean. Like any RPC method, an error can be thrown.
  const wasAdded = await ethereum.request({
    method: 'wallet_watchAsset',
    params: {
      type: 'ERC1155',
      options: {
        address,
        tokenId,
      },
    },
  });

  if (wasAdded) {
    console.log('User successfully added the token!');
  } else {
    throw new Error("Token was not added")
  }
}
