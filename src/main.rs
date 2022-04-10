use web3::types::{BlockId, BlockNumber, U64};

#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = web3::transports::http::Http::new("https://rpc.api.moonbeam.network")?;
    let web3 = web3::Web3::new(transport);

    let current = web3.eth().block_number().await?;

    let mut number: U64 = U64([190220]);

    while current > number {
    
    println!("{}", number);

    let blocknumber = BlockNumber::Number(number);
    let block = BlockId::Number(blocknumber);

    let blockdata = web3.eth().block(block).await?.unwrap();
    let transactions = blockdata.transactions;

    for i in transactions {
        let receipt = web3.eth().transaction_receipt(i).await?.unwrap();
        if receipt.contract_address != None {
            println!("{:?}", receipt.contract_address.unwrap());
        }
    }
    number = number + 1;
}
    Ok(())
}