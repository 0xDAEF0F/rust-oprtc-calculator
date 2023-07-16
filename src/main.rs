use dotenv::dotenv;
use ethers::{
    contract::abigen,
    core::types::Address,
    providers::{Middleware, Provider, StreamExt, Ws},
    types::Filter,
};
use eyre::Result;
use std::env;
use std::sync::Arc;

abigen!(LendingVault, "LendingVault.json");

const VAULT_ADDRESS: &str = "0xaF53431488E871D103baA0280b6360998F0F9926";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let wss_url = env::var("WSS_URL").expect("Env: WSS_URL");

    let provider = Provider::<Ws>::connect(wss_url).await?;
    let client = Arc::new(provider);
    let address: Address = VAULT_ADDRESS.parse()?;
    let contract = LendingVault::new(address, client);

    let deposit_logs = provider.get_logs(&dep_filter).await?;

    listen_all_events(&contract).await?;

    Ok(())
}

async fn listen_all_events(contract: &LendingVault<Provider<Ws>>) -> Result<()> {
    let events = contract.events().from_block(17564663);
    let mut stream = events.stream().await?.take(1);

    while let Some(Ok(evt)) = stream.next().await {
        match evt {
            LendingVaultEvents::DepositFilter(f) => println!("{f:?}"),
            LendingVaultEvents::WithdrawFilter(f) => println!("{f:?}"),
            _ => println!("dont care."),
        }
    }

    Ok(())
}
