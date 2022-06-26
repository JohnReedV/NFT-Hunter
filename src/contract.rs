use web3::{api::Web3, contract::{Contract, Options}, transports::http::Http, types::{Address, H160}};
use std::fs::File;

pub async fn get_contract(web3: &Web3<Http>, contract_address: H160) {
    let is721: bool;
    let is1155: bool;

    let data721: Option<bool> = check721(web3, contract_address).await.ok();

    match data721 {
        Some(result) => { if result == true {is721 = true} else {is721 = false} }
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
