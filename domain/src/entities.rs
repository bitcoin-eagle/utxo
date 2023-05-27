use nameof::*;

use crate::{events::*, value_objects::*};

#[readonly::make]
pub struct UtxoEntity {
    pub id: UtxoId,
    pub value: u64,
    pub spendable: bool,
    pub confirmed_block_height: usize,
    pub confirmed_block_hash: BlockHash,
    pub spent_block_height: Option<usize>,
    pub spent_block_hash: Option<BlockHash>,
}

impl UtxoEntity {
    pub fn new(id: UtxoId) -> Self {
        Self {
            id,
            spendable: false,
            value: 0,
            confirmed_block_height: 0,
            confirmed_block_hash: Default::default(),
            spent_block_height: None,
            spent_block_hash: None,
        }
    }
    pub fn apply(&mut self, event: &ChainStateEvent) {
        match event {
            ChainStateEvent::UtxoConfirmed {
                utxo_id: _,
                value,
                confirmed_block_height,
                confirmed_block_hash,
            } => {
                self.spendable = true;
                self.value = *value;
                self.confirmed_block_height = *confirmed_block_height;
                self.confirmed_block_hash = confirmed_block_hash.clone();
            }
            ChainStateEvent::UtxoUnconfirmed { utxo_id: _ } => {
                self.spendable = false;
                self.confirmed_block_height = 0;
                self.confirmed_block_hash = Default::default();
            }
            ChainStateEvent::UtxoSpent {
                utxo_id,
                tx_id,
                spent_block_height,
                spent_block_hash,
            } => todo!(),
            ChainStateEvent::UtxoUnspent { utxo_id } => todo!(),
            _ => debug_assert!(
                false,
                "unknown {} event: {:?}",
                name_of_type!(UtxoEntity),
                event
            ),
        }
    }
}
