use std::fmt::Error;

use web3::{api::Web3, transports::http::Http, types::BlockId, types::BlockNumber, types::H160, types::U64};

pub async fn get_contract_address(web3: &Web3<Http>, number: U64) -> web3::Result<(web3::types::H160, Error)> {
    //let contract = web3::contract::Contract::new();

    let mut result: H160 = H160::default();

        let blocknumber = BlockNumber::Number(number);
        let block = BlockId::Number(blocknumber);

        let blockdata = web3.eth().block(block).await?;

        match blockdata {
            Some(data) => {
                let transactions = data.transactions;

                for i in transactions {
                    let receipt = web3.eth().transaction_receipt(i).await?;

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
