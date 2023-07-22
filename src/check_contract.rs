#![allow(unused_variables)]
use hex_literal::hex;
use std::fs::File;
use web3::{
    api::Web3,
    contract::{Contract, Options},
    transports::http::Http,
    types::{Address, H160, U256},
};

pub async fn check_contract(
    web3: &Web3<Http>,
    contract_address: H160,
) -> Option<web3::contract::Contract<Http>> {
    let mut is721: bool = false;
    let mut is1155: bool = false;

    let data721: Option<bool> = check721(web3, contract_address).await.ok();
    match data721 {
        Some(result) => {
            if result == true {
                is721 = true
            }
        }
        None => {}
    }

    let data1155: Option<bool> = check1155(web3, contract_address).await.ok();
    match data1155 {
        Some(result) => {
            if result == true {
                is1155 = true
            }
        }
        None => {}
    }

    if is721 == true {
        println!("Contract Type: 721, Address: {:?}", contract_address);
        let file = File::open("./ABIs/721ABI.json").unwrap();
        let abi = web3::ethabi::Contract::load(file).unwrap();
        return Some(Contract::new(web3.eth(), contract_address, abi));
    } else if is1155 == true {
        println!("Contract Type: 1155, Address: {:?}", contract_address);
        let file = File::open("./ABIs/1155ABI.json").unwrap();
        let abi = web3::ethabi::Contract::load(file).unwrap();
        return Some(Contract::new(web3.eth(), contract_address, abi));
    } else {
        return None;
    }
}

pub async fn check721(web3: &Web3<Http>, contract_address: H160) -> web3::contract::Result<bool> {
    let token_id: u64 = 1;
    let file = File::open("./ABIs/721ABI.json").unwrap();
    let abi = web3::ethabi::Contract::load(file).unwrap();
    let contract = Contract::new(web3.eth(), contract_address, abi);

    let result = contract.query("ownerOf", token_id, None, Options::default(), None);
    let owner: Address = result.await?;

    Ok(true)
}

pub async fn check1155(web3: &Web3<Http>, contract_address: H160) -> web3::contract::Result<bool> {
    let token_id: u64 = 1;
    let my_address: H160 = hex!("A07876136c3A3141Dc2C25071330D3B08225e043").into();
    let file = File::open("./ABIs/1155ABI.json").unwrap();
    let abi = web3::ethabi::Contract::load(file).unwrap();
    let contract = Contract::new(web3.eth(), contract_address, abi);

    let result = contract.query(
        "balanceOf",
        (my_address, token_id),
        None,
        Options::default(),
        None,
    );
    let amount: U256 = result.await?;

    Ok(true)
}
