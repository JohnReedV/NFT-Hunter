use std::fmt::Error;

use web3::{
    api::Web3,
    transports::http::Http,
    types::{Block, H160, H256},
};

pub async fn get_contract_address(
    web3: &Web3<Http>,
    blockdata: &Option<Block<H256>>,
) -> web3::Result<(web3::types::H160, Error)> {
    //let contract = web3::contract::Contract::new();

    let mut result: H160 = H160::default();

    match blockdata {
        Some(data) => {
            let transactions = &data.transactions;

            for i in transactions {
                let receipt = web3.eth().transaction_receipt(*i).await?;

                match receipt {
                    Some(data) => {
                        if data.contract_address != None {
                            result = data.contract_address.unwrap();
                        }
                    }
                    None => {}
                }
            }
        }
        None => {}
    }
    Ok((result, Error))
}
