# **NFT-Hunter**

**Creating NFT hunter in Rust!**

## **Result**

NFT-Hunter will start at the given start block and on the given chain.

From this point it will search every block for contracts.

When a contract is identified it will determine if it is an NFT contract.

All NFTs of identified contracts will be found in the upcoming blocks as they are searched.

If a change in owner is made for example, NFT-Hunter will update that property on the given NFT entity.
