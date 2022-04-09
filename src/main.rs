use web3::types::{BlockId, BlockNumber};

#[tokio::main]
async fn main() -> web3::Result<()> {
    let transport = web3::transports::http::Http::new(env!("infura_string"))?;
    let web3 = web3::Web3::new(transport);

    let number = web3.eth().block_number().await?;

    let blocknumber = BlockNumber::Number(number);
    let block = BlockId::Number(blocknumber);

    let _blockdata = web3.eth().block(block).await?;

    let pool = web3.txpool().content().await?;

    println!("{:?}", pool);



    Ok(())
}