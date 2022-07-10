use std::{fmt::Error};
use hex_literal::hex;
use web3::{
    api::Web3,
    transports::Http,
    contract::Options,
    types::{Block, H160, H256},
};

pub struct Nft {
    pub world: H160,
    pub owner: H160,
    pub tokenid: u64,
    pub tokenuri: String
}

pub async fn get_nfts(
    web3: &Web3<Http>,
    contracts: &Vec<web3::contract::Contract<Http>>,
    blockdata: &Option<Block<H256>>,
) -> web3::contract::Result<(Vec<Nft>, Error)> {
    let mut nft_list: Vec<Nft> = Vec::new();
    let transfer_topic: H256 = hex!("ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef").into();
    match blockdata {
        Some(data) => {
            let transactions = &data.transactions;
            for transaction in transactions {
                let receipt = web3.eth().transaction_receipt(*transaction).await?;
                for contract in contracts {
                    let address = contract.address();
                    match &receipt {
                        Some(result) => match result.contract_address {
                            Some(info) => {
                                if info == address {
                                    let logs = &result.logs;
                                    for log in logs {
                                        let topics = &log.topics;
                                        if topics[0] == transfer_topic {
                                            let token_uri: String;
                                            let token_id = H256::from(topics[3]).to_low_u64_be();
                                            let result = contract.query("tokenURI", token_id, None, Options::default(), None);

                                            //if error try 1155 case
                                            match result.await {
                                                Ok(tokenuri) => { 
                                                    token_uri = tokenuri;

                                                 } Err(err) => {
                                                    let result = contract.query("uri", token_id, None, Options::default(), None);
                                                    token_uri = result.await?;
                                                 }
                                            }

                                            let nft = Nft {
                                                world: address,
                                                owner: H160::from(topics[2]),
                                                tokenid: token_id,
                                                tokenuri: token_uri
                                            };
                                            println!("Contract: {:?}, Owner: {:?}, TokenID: {:?}, tokenURI: {:?}, ", nft.world, nft.owner, nft.tokenid, nft.tokenuri);
                                            nft_list.push(nft);
                                        }
                                    }
                                }
                            }
                            None => {}
                        },
                        None => {}
                    }
                }
            }
        }
        None => {}
    }
    Ok((nft_list, Error))
}
