use crate::{entities::*, events::*, value_objects::*};

use bitcoin::Network;
use std::collections::HashMap;

#[readonly::make]
pub struct ChainStateAggregate {
    pub network: Network,
    pub utxos: HashMap<UtxoId, UtxoEntity>,
}

impl ChainStateAggregate {
    pub fn apply(&mut self, event: ChainStateEvent) {
        match event {
            ChainStateEvent::UtxoConfirmed {
                utxo_id,
                value,
                confirmed_block_height,
                confirmed_block_hash,
            } => todo!(),
            ChainStateEvent::UtxoUnconfirmed { utxo_id } => todo!(),
            ChainStateEvent::UtxoSpent {
                utxo_id,
                tx_id,
                spent_block_height,
                spent_block_hash,
            } => todo!(),
            ChainStateEvent::UtxoUnspent { utxo_id } => todo!(),
        }
    }
}

impl Default for ChainStateAggregate {
    fn default() -> Self {
        Self {
            network: Network::Testnet,
            utxos: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use bitcoin::OutPoint;

    use super::*;

    static utxo_id_a: UtxoId = UtxoId::new(OutPoint::new(
        bitcoin::Txid::from_str("a00b59c6fc60a3ed7498d7dbf963aeea9bdbabdf7691f37919d284d08619f643")
            .unwrap(),
        0,
    ));

    static utxo_id_b: UtxoId = UtxoId::new(OutPoint::new(
        bitcoin::Txid::from_str("b0027cd224d79ee1a72aa15a35af6257bc45e27ce768d7bd20c3935134793a50")
            .unwrap(),
        0,
    ));

    static block_hash_a: BlockHash = BlockHash::new(
        bitcoin::BlockHash::from_str(
            "0000a003df9e5e766fbeb52c564ca988f9add78949ced2892a5cdf222fd16497",
        )
        .unwrap(),
    );

    #[test]
    fn new() {
        let mut a = ChainStateAggregate::default();
        a.apply(ChainStateEvent::UtxoConfirmed {
            utxo_id: utxo_id_a,
            value: 10,
            confirmed_block_height: 1,
            confirmed_block_hash: block_hash_a,
        });
    }
}
