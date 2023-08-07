use std::{ops::Add, sync::Arc, vec};

use anyhow::Result;
use artemis_core::types::Strategy;
use async_trait::async_trait;
use bindings::counter::Counter;
use ethers::{
    providers::Middleware,
    signers::Signer,
    types::{Address, H256},
};
use mev_share::{
    rpc::{BundleItem, Inclusion, SendBundleRequest},
    sse,
};
use tracing::info;

/// Event which is the input to the current strategy.
#[derive(Debug, Clone)]
pub enum Event {
    MEVShareEvent(sse::Event),
}

/// Action which is the output of the current strategy.
#[derive(Debug, Clone)]
pub enum Action {
    SubmitBundle(SendBundleRequest),
}

#[derive(Debug, Clone)]
pub struct SimpleBackrunner<M, S> {
    /// Ethers client.
    client: Arc<M>,
    /// Signer for transactions.
    tx_signer: S,
    /// Contract instance.
    contract: Counter<M>,
}

impl<M: Middleware + 'static, S: Signer> SimpleBackrunner<M, S> {
    pub fn new(client: Arc<M>, tx_signer: S, contract_address: Address) -> Self {
        Self { client: client.clone(), tx_signer, contract: Counter::new(contract_address, client) }
    }
}

#[async_trait]
impl<M: Middleware + 'static, S: Signer + 'static> Strategy<Event, Action>
    for SimpleBackrunner<M, S>
{
    /// This function is called before the strategy starts. You can use it to
    /// load initial state which can be used to process events.
    async fn sync_state(&mut self) -> Result<()> {
        info!("Done syncing state");
        Ok(())
    }

    // This function is used to process mev share events. It should return a
    // vector of bundles to submit to the matchmaker.
    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            Event::MEVShareEvent(event) => {
                info!("Received mev share event: {:?}", event);
                let bundles = self.generate_bundles(event.hash).await;
                return bundles.into_iter().map(Action::SubmitBundle).collect();
            }
        }
    }
}

// Example of how to construct a bundle.
impl<M: Middleware + 'static, S: Signer + 'static> SimpleBackrunner<M, S> {
    /// We backrun the transaction with a counter increment.
    pub async fn generate_bundles(&self, tx_hash: H256) -> Vec<SendBundleRequest> {
        // generate and sign our tx
        let tx = self.contract.increment().tx;
        let signature = self.tx_signer.sign_transaction(&tx).await.unwrap();
        let bytes = tx.rlp_signed(&signature);
        let txs = vec![
            BundleItem::Hash { hash: tx_hash },
            BundleItem::Tx { tx: bytes, can_revert: false },
        ];
        let block_num = self.client.get_block_number().await.unwrap();
        let bundle = SendBundleRequest {
            bundle_body: txs,

            inclusion: Inclusion { block: block_num.add(1), max_block: Some(block_num.add(30)) },
            ..Default::default()
        };
        info!("generating bundle: {:?}", bundle);
        // add bundle here if you want to submit it to the matchmaker
        vec![]
    }
}
