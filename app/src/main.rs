use bindings::my_contract::MyContract;

use ethers::{providers::Provider, prelude::Middleware, types::Address};

use std::convert::TryFrom;
use std::sync::Arc;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::try_from("https://eth-rinkeby.alchemyapi.io/v2/Lc7oIGYeL_QvInzI0Wiu_pOZZDEKBrdf")?;
    let provider = Arc::new(provider);

    let address = "0x0000000000000000000000000000000000000000".parse::<Address>()?;

    let contract = MyContract::new(address, provider);
    let blk = contract.client().get_block_number().await?;
    println!("Hello, world! {}", blk);
    Ok(())
}
