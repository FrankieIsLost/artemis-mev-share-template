use std::sync::Arc;

use artemis_core::{
    collectors::mevshare_collector::MevShareCollector,
    engine::Engine,
    executors::mev_share_executor::MevshareExecutor,
    types::{CollectorMap, ExecutorMap},
};
use ethers::{
    prelude::{rand, MiddlewareBuilder},
    providers::{Provider, Ws},
    signers::{LocalWallet, Signer},
    types::Address,
};

use simple_backrun::{Action, Event, SimpleBackrunner};
use tracing::{info, Level};
use tracing_subscriber::{filter, prelude::*};

mod simple_backrun;

use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let filter = filter::Targets::new()
        .with_target("bot", Level::INFO)
        .with_target("artemis_core", Level::INFO);
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer()).with(filter).init();
    // Set up engine.
    let mut engine: Engine<Event, Action> = Engine::default();

    // Set up collector.
    let mevshare_collector =
        Box::new(MevShareCollector::new(String::from("https://mev-share.flashbots.net")));
    let mevshare_collector = CollectorMap::new(mevshare_collector, Event::MEVShareEvent);
    engine.add_collector(Box::new(mevshare_collector));

    // Set up strategy.

    let ws = Ws::connect("wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27").await?;
    let provider = Provider::new(ws);

    // These should be replaced with actual values
    let wallet: LocalWallet = ethers::signers::LocalWallet::new(&mut rand::thread_rng());
    let address = wallet.address();
    let contract_address = Address::random();

    let provider = Arc::new(provider.nonce_manager(address).with_signer(wallet.clone()));
    let strategy = SimpleBackrunner::new(Arc::new(provider), wallet, contract_address);
    engine.add_strategy(Box::new(strategy));

    // Set up executor.

    // this should be the key you want to use to sign flashbots bundle
    let fb_signer = ethers::signers::LocalWallet::new(&mut rand::thread_rng());

    let mev_share_executor = Box::new(MevshareExecutor::new(fb_signer));
    let mev_share_executor = ExecutorMap::new(mev_share_executor, |action| match action {
        Action::SubmitBundle(bundle) => Some(bundle),
    });
    engine.add_executor(Box::new(mev_share_executor));

    // Start engine.
    if let Ok(mut set) = engine.run().await {
        while let Some(res) = set.join_next().await {
            info!("res: {:?}", res);
        }
    }

    Ok(())
}
