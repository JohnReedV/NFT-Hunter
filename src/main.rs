use web3::types::{BlockId, BlockNumber, U64};

#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = web3::transports::http::Http::new("http://127.0.0.1:8545")?;
    let web3 = web3::Web3::new(transport);

    let current = web3.eth().block_number().await?;

    let mut number: U64 = U64([14550000]);

    while current > number {
    
    println!("{}", number);

    let blocknumber = BlockNumber::Number(number);
    let block = BlockId::Number(blocknumber);

    let blockdata = web3.eth().block(block).await?;

    match blockdata {
        Some(data) => {
            let transactions = data.transactions;

            for i in transactions {
                let receipt = web3.eth().transaction_receipt(i).await?;

                match receipt {
                    Some(data) => {
                        if data.contract_address != None {
                            println!("{:?}", data.contract_address.unwrap());
                        }
                    }
                    None => {}
                }
            }
        }
        None => {}
    }

    number = number + 1;
}
    Ok(())
}