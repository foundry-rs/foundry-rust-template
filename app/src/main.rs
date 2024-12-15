use alloy::{
    primitives::U256,
    providers::builder,
};
use eyre::Result;
use bindings::counter::Counter;

#[tokio::main]
async fn main() -> Result<()> {
    let provider = builder().with_recommended_fillers().on_anvil_with_wallet();

    let deployed_contract = Counter::deploy(provider.clone()).await?;
    println!("Deployed contract at address: {:?}", deployed_contract.address());

    let initial_number = deployed_contract.number().call().await?;
    println!("Initial number: {}", initial_number._0);

    let new_number = U256::from(123);
    let set_number_tx = deployed_contract.setNumber(new_number).send().await?;
    println!("setNumber called, transaction hash: {:?}", set_number_tx.tx_hash());

    let updated_number = deployed_contract.number().call().await?;
    println!("Updated number after setting: {}", updated_number._0);

    let increment_tx = deployed_contract.increment().send().await?;
    println!("increment called, transaction hash: {:?}", increment_tx.tx_hash());

    let incremented_number = deployed_contract.number().call().await?;
    println!("Number after incrementing: {}", incremented_number._0);
    Ok(())
}
