use std::sync::Arc;
use tdn::smol::lock::RwLock;

use crate::transaction::Transaction;

pub struct Round {
    //
}

pub struct Block {
    hash: [u8; 32],
    txs: Vec<Transaction>,
    bio: [u8; 256],
}

pub struct Pool {
    pub txs: Vec<Transaction>,
}

pub async fn start_miner(pool: Arc<RwLock<Pool>>) -> Result<Block, ()> {
    let mut pool_lock = pool.write().await;
    let txs: Vec<_> = pool_lock.txs.drain(..).collect();
    drop(pool_lock);

    let block = Block {
        hash: [0u8; 32],
        bio: [0u8; 256],
        txs,
    };

    //

    Ok(block)
}
