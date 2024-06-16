use alloy::{
    primitives::Address,
    providers::{builder, Provider},
};
use eyre::Result;
use foundry_contracts::counter::Counter;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = builder().with_recommended_fillers().on_anvil_with_wallet();

    let address = "0x0000000000000000000000000000000000000000".parse::<Address>()?;

    let _contract = Counter::new(address, provider.clone());

    let blk = provider.get_block_number().await?;
    println!("Hello, world! {}", blk);
    Ok(())
}
