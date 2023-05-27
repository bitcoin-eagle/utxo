use bitcoin::{constants::ChainHash, hashes::Hash};

use crate::{entities::*, events::*, value_objects::*};

use std::collections::{hash_map::Entry, HashMap};

#[readonly::make]
pub struct ChainStateAggregate {
    pub network: Network,
    pub genesis_block_hash: BlockHash,
    pub utxos: HashMap<UtxoId, UtxoEntity>,
}

impl ChainStateAggregate {
    pub fn apply(&mut self, event: &ChainStateEvent) {
        match event {
            ChainStateEvent::ChainInitialized {
                genesis_block_hash,
                network,
            } => {
                self.network = *network;
                self.genesis_block_hash = genesis_block_hash.clone();
            }
            ChainStateEvent::UtxoConfirmed {
                utxo_id,
                value,
                confirmed_block_height,
                confirmed_block_hash,
            } => match self.utxos.entry(*utxo_id) {
                Entry::Occupied(mut entry) => entry.get_mut().apply(event),
                Entry::Vacant(entry) => entry.insert(UtxoEntity::new(*utxo_id)).apply(event),
            },
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
            network: Network::Regtest,
            genesis_block_hash: BlockHash::new(bitcoin::BlockHash::all_zeros()),
            utxos: Default::default(),
        }
    }
}

#[cfg(test)]
mod chain_state_aggregate_tests {
    use bitcoin::{
        constants::ChainHash,
        hashes::{sha256d, Hash},
        OutPoint,
    };
    use std::str::FromStr;

    use super::*;

    fn utxo_id_a() -> UtxoId {
        UtxoId::new(OutPoint::new(
            bitcoin::Txid::from_str(
                "a00b59c6fc60a3ed7498d7dbf963aeea9bdbabdf7691f37919d284d08619f643",
            )
            .unwrap(),
            0,
        ))
    }

    fn utxo_id_b() -> UtxoId {
        UtxoId::new(OutPoint::new(
            bitcoin::Txid::from_str(
                "b0027cd224d79ee1a72aa15a35af6257bc45e27ce768d7bd20c3935134793a50",
            )
            .unwrap(),
            0,
        ))
    }

    fn block_hash_a() -> BlockHash {
        BlockHash::new(
            bitcoin::BlockHash::from_str(
                "0000a003df9e5e766fbeb52c564ca988f9add78949ced2892a5cdf222fd16497",
            )
            .unwrap(),
        )
    }

    #[test]
    fn apply_initialized() {
        // Arrange
        let mut a = ChainStateAggregate::default();
        let default_network = a.network;
        let default_genesis_bytes = a.genesis_block_hash.0.to_byte_array();

        // Act
        a.apply(&ChainStateEvent::ChainInitialized {
            genesis_block_hash: BlockHash::new(bitcoin::BlockHash::from_byte_array(
                ChainHash::TESTNET.to_bytes(),
            )),
            network: bitcoin::Network::Testnet,
        });

        // Assert
        assert_eq!(a.network, bitcoin::Network::Testnet);
        assert_ne!(a.network, default_network);
        assert_eq!(
            a.genesis_block_hash.0.to_string(),
            "000000000933ea01ad0ee984209779baaec3ced90fa3f408719526f8d77f4943"
        );
        assert_ne!(
            a.genesis_block_hash.0.to_byte_array(),
            default_genesis_bytes
        );
    }

    #[test]
    fn apply_utxoconfirmed() {
        // Arrange
        let mut a = ChainStateAggregate::default();
        a.apply(&ChainStateEvent::ChainInitialized {
            genesis_block_hash: BlockHash::new(bitcoin::BlockHash::from_byte_array(
                ChainHash::TESTNET.to_bytes(),
            )),
            network: bitcoin::Network::Testnet,
        });
        assert!(a.utxos.get(&utxo_id_a()).is_none());

        // Act
        a.apply(&ChainStateEvent::UtxoConfirmed {
            utxo_id: utxo_id_a(),
            value: 10,
            confirmed_block_height: 1,
            confirmed_block_hash: block_hash_a(),
        });

        // Assert
        let entity = a.utxos.get(&utxo_id_a());
        assert!(entity.is_some());
        let entity = entity.unwrap();
        assert_eq!(entity.id, utxo_id_a());
        assert_eq!(entity.value, 10);
        assert_eq!(entity.confirmed_block_height, 1);
        assert_eq!(entity.confirmed_block_hash, block_hash_a());
    }
}
