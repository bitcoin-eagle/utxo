use crate::value_objects::*;

pub enum ChainStateEvent {
    /// Spendable: UTXO has been confirmed in a block in a best chain
    UtxoConfirmed {
        utxo_id: UtxoId,
        value: u64,
        confirmed_block_height: usize,
        confirmed_block_hash: BlockHash,
    },
    /// Not-Spendable: UTXO has been reorged out of current best chain after it had been previously confirmed
    UtxoUnconfirmed { utxo_id: UtxoId },
    /// Not-Spendable: UTXO spend has been confirmed in a block in a best chain
    UtxoSpent {
        utxo_id: UtxoId,
        tx_id: Txid,
        spent_block_height: usize,
        spent_block_hash: BlockHash,
    },
    /// Spendable: UTXO spend has been reorged out of current best chain after it had been previously confirmed spent
    UtxoUnspent { utxo_id: UtxoId },
}
