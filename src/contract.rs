use web3::{api::Web3, contract::{Contract, Options}, transports::http::Http, types::{Address, H160, U256}};
use hex_literal::hex;
use std::fs::File;

pub async fn get_contract(web3: &Web3<Http>, contract_address: H160) {
    let is721: bool;
    let is1155: bool;

    let data721: Option<bool> = check721(web3, contract_address).await.ok();
    match data721 {
        Some(result) => { if result == true {is721 = true} else {is721 = false} }
        None => {}
    }

    let data1155: Option<bool> = check721(web3, contract_address).await.ok();
    match data1155 {
        Some(result) => { if result == true {is1155 = true} else {is1155 = false} }
        None => {}
    }

}

pub async fn check721(web3: &Web3<Http>, contract_address: H160) -> web3::contract::Result<bool> {
    let tokenID: u64 = 1;
    let mut file = File::open("./ABIs/721ABI.json").unwrap();
    let abi = web3::ethabi::Contract::load(file).unwrap();
    let contract = Contract::new(web3.eth(), contract_address, abi);

    let result = contract.query("ownerOf", tokenID, None, Options::default(), None);
    let owner: Address = result.await?;

    Ok(true)
}

pub async fn check1155(web3: &Web3<Http>, contract_address: H160) -> web3::contract::Result<bool> {
    let tokenID: u64 = 1;
    let my_address: H160 = hex!("A07876136c3A3141Dc2C25071330D3B08225e043").into();
    let mut file = File::open("./ABIs/1155ABI.json").unwrap();
    let abi = web3::ethabi::Contract::load(file).unwrap();
    let contract = Contract::new(web3.eth(), contract_address, abi);

    let result = contract.query("balanceOf", (my_address, tokenID), None, Options::default(), None);
    let amount: U256 = result.await?;

    Ok(true)
}
