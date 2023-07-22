#![allow(unused_variables)]
mod contract_address_finder;
use contract_address_finder::get_contract_address;
mod check_contract;
use check_contract::check_contract;
mod get_nfts;
use get_nfts::{get_nfts, Nft};
use std::collections::HashSet;
use web3::{
    contract::Contract,
    transports::Http,
    types::{BlockId, BlockNumber, H160, U64},
};

#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = web3::transports::http::Http::new("https://rpc.api.moonbeam.network")?;
    let web3 = web3::Web3::new(transport);

    let mut block_number: U64 = U64([209449]);

    let mut contracts: Vec<web3::contract::Contract<Http>> = Vec::new();
    let mut nft_list: Vec<Nft> = Vec::new();

    let mut processed_contracts: HashSet<H160> = HashSet::new();

    loop {
        block_number = block_number + U64([1]);

        println!("{}", block_number);
        let blocknumber = BlockNumber::Number(block_number);
        let block = BlockId::Number(blocknumber);
        let blockdata = web3.eth().block(block).await?;

        let data = get_contract_address(&web3, &blockdata).await?;

        match data {
            Some(contract_address) => {
                if !processed_contracts.contains(&contract_address) {
                    let info: Option<Contract<Http>> =
                        check_contract(&web3, contract_address).await;
                    match info {
                        Some(contract) => {
                            contracts.push(contract);
                            processed_contracts.insert(contract_address);
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }

        let nfts: Result<(Vec<Nft>, std::fmt::Error), web3::contract::Error> =
            get_nfts(&web3, &contracts, &blockdata).await;

        match nfts {
            Ok(result) => {
                let nft_vec = result.0;
                for nft in nft_vec {
                    let mut updated = false;
                    let mut i = 0;
                    let length = nft_list.len();
                    loop {
                        if !i < length {
                            if nft_list[i].world == nft.world && nft_list[i].tokenid == nft.tokenid
                            {
                                nft_list[i].owner = nft.owner;
                                updated = true;
                                break;
                            }
                            i = i + 1;
                        } else {
                            break;
                        };
                    }
                    if !updated {
                        nft_list.push(nft)
                    }
                }
            }
            Err(err) => {}
        }
    }
}
