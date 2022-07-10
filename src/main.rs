#![allow(unused_variables)]
mod contract_address_finder;
use contract_address_finder::get_contract_address;
mod check_contract;
use check_contract::check_contract;
mod get_nfts;
use get_nfts::{get_nfts, Nft};
use web3::{contract::Contract, transports::Http, types::{H160, U64, BlockNumber, BlockId}};

#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = web3::transports::http::Http::new("https://rpc.api.moonbeam.network")?;
    let web3 = web3::Web3::new(transport);

    let mut block_number: U64 = U64([209449]);

    let mut contracts: Vec<web3::contract::Contract<Http>> = Vec::new();
    let mut nft_list: Vec<Nft> = Vec::new();

    loop {
        block_number = block_number + U64([1]);
        
        println!("{}", block_number);
        let blocknumber = BlockNumber::Number(block_number);
        let block = BlockId::Number(blocknumber);
        let blockdata = web3.eth().block(block).await?;

        let data = get_contract_address(&web3, &blockdata).await?;

        let contract_address = data.0;
        let info: Option<Contract<Http>> = check_contract(&web3, contract_address).await;
        match info {
            Some(contract) => {
                if data.0 != H160::default() {
                    contracts.push(contract);
                }
            } None => {}
        }
        let nfts = get_nfts(&web3, &contracts, &blockdata).await;

        match nfts {
            Ok(result) => {
                let nft_vec = result.0;
                for nft in nft_vec {
                    nft_list.push(nft);
                }

             } Err(err) => {}
        }
    }
}