use web3::{api::Web3, contract::Contract, transports::Http, types::{H160, U64, BlockNumber, BlockId}};
use std::fmt::Error;

pub async fn get_nfts(web3: &Web3<Http>, contract: Contract<Http>, block_number: U64) -> web3::Result<(String, Error)> {
    let address: H160 = contract.address();

    let blocknumber = BlockNumber::Number(block_number);
    let block = BlockId::Number(blocknumber);
    let blockdata = web3.eth().block(block).await?;

    let mut dog: String = String::new();
    match blockdata {
        Some(data) => {
            let bloomdata = data.logs_bloom;
            match bloomdata {
                Some(bloom) => {
                    let log = bloom.to_string();
                    
                } None => {}
            }
        } None => {}
    }
    Ok((dog, Error))
}