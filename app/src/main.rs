use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder},
};
use bindings::wavsservicemanager::WavsServiceManager;
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = ProviderBuilder::new().on_builtin("http://127.0.0.1:8545").await?;

    let address = "0x0000000000000000000000000000000000000000".parse::<Address>()?;

    let _contract = WavsServiceManager::new(address, provider.clone());

    let blk = provider.get_block_number().await?;
    println!("Hello, world! {}", blk);
    Ok(())
}
