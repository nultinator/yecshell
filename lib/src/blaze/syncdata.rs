use std::sync::Arc;

use http::Uri;
use tokio::sync::RwLock;

use super::{block_witness_data::BlockAndWitnessData, sync_status::SyncStatus};
use crate::compact_formats::TreeState;
use crate::lightwallet::WalletOptions;
use crate::{lightclient::lightclient_config::LightClientConfig, lightwallet::data::BlockData};

pub struct BlazeSyncData {
    pub(crate) sync_status: Arc<RwLock<SyncStatus>>,
    pub(crate) block_data: BlockAndWitnessData,
    uri: Uri,
    pub(crate) wallet_options: WalletOptions,
}

impl BlazeSyncData {
    pub fn new(config: &LightClientConfig) -> Self {
        let sync_status = Arc::new(RwLock::new(SyncStatus::default()));

        Self {
            sync_status: sync_status.clone(),
            uri: config.server.clone(),
            block_data: BlockAndWitnessData::new(config, sync_status),
            wallet_options: WalletOptions::default(),
        }
    }

    pub fn uri(&self) -> &'_ Uri {
        &self.uri
    }

    pub async fn setup_for_sync(
        &mut self,
        start_block: u64,
        end_block: u64,
        batch_num: usize,
        existing_blocks: Vec<BlockData>,
        verified_tree: Option<TreeState>,
        wallet_options: WalletOptions,
    ) {
        if start_block < end_block {
            panic!("Blocks should be backwards");
        }

        // Clear the status for a new sync batch
        self.sync_status
            .write()
            .await
            .new_sync_batch(start_block, end_block, batch_num);
        
        self.wallet_options = wallet_options;

        self.block_data.setup_sync(existing_blocks, verified_tree).await;
    }

    // Finish up the sync
    pub async fn finish(&self) {
        self.sync_status.write().await.finish();
    }
}
