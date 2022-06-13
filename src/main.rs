mod contract_address_finder;
use contract_address_finder::get_contract_address;

mod contract;
use contract::get_contract;

use web3::types::{U64, H160};


#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = web3::transports::http::Http::new("https://rpc.api.moonbeam.network")?;
    let web3 = web3::Web3::new(transport);

    let current: U64 = web3.eth().block_number().await?;
    let mut number: U64 = U64([190220]);

    while current > number {
    println!("{}", number);

    let data = get_contract_address(&web3, number).await?;
    
    if data.0 == H160([0, 1, 2, 3, 4, 5, 6 ,7 ,8, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]) {
        number = number + U64([1]);
        continue;
     }
    
    let contract_address = data.0;
    println!("{:?}", contract_address);

    let contract = get_contract(&web3, contract_address).await;

    number = number + U64([1]);
    }
    
    Ok(())
}