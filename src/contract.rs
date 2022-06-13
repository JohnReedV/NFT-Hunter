use web3::{api::Web3, contract::{Contract, Options}, transports::http::Http, types::H160};
use std::fs::File;

pub async fn get_contract(web3: &Web3<Http>, contract_address: H160) {

    let erc721: bool = check721(web3, contract_address);

}

pub fn check721(web3: &Web3<Http>, contract_address: H160) -> bool {
    let mut file = File::open("../ABIs/721ABI.json").unwrap();
    let abi = web3::ethabi::Contract::load(file).unwrap();
    let contract = Contract::new(web3.eth(), contract_address, abi);

   // let result = contract.query("ownerOf", params, from, options, block)

   // if (result) { return true; } else { return false; }

    }

