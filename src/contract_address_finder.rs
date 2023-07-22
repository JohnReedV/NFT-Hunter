
use web3::{
    api::Web3,
    transports::http::Http,
    types::{Block, H160, H256},
};

pub async fn get_contract_address(
    web3: &Web3<Http>,
    blockdata: &Option<Block<H256>>,
) -> web3::Result<Option<H160>> {

    if let Some(data) = blockdata {
        for tx in &data.transactions {
            if let Some(receipt) = web3.eth().transaction_receipt(*tx).await? {
                if let Some(contract_address) = receipt.contract_address {
                    return Ok(Some(contract_address));
                }
            }
        }
    }
    Ok(None)
}

