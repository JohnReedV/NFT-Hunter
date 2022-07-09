#![allow(unused_variables)]
mod contract_address_finder;
use contract_address_finder::get_contract_address;
mod check_contract;
use check_contract::check_contract;
use web3::types::{H160, U64};

#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = web3::transports::http::Http::new("https://rpc.api.moonbeam.network")?;
    let web3 = web3::Web3::new(transport);

    let mut current_block: U64 = web3.eth().block_number().await?;
    let mut block_number: U64 = U64([1400000]);

    loop {
        println!("{}", block_number);

        let data = get_contract_address(&web3, block_number).await?;
        if data.0 == H160::default() {
            block_number = block_number + U64([1]);
            continue;
        }

        let contract_address = data.0;
        println!("{:?}", contract_address);

        let contract = check_contract(&web3, contract_address).await;

        block_number = block_number + U64([1]);
        current_block = web3.eth().block_number().await?;
    }

    Ok(())
}
