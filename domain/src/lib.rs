pub enum ChainStateEvent {
    /// Spendable: UTXO has been confirmed in a block in a best chain
    UtxoConfirmed {},
    /// Not-Spendable: UTXO has been reorged out of current best chain after it had been previously confirmed
    UtxoUnconfirmed {},
    /// Not-Spendable: UTXO spend has been confirmed in a block in a best chain
    UtxoSpent {},
    /// Spendable: UTXO spend has been reorged out of current best chain after it had been previously confirmed spent
    UtxoUnspent {},
}

pub struct UtxoEntity {}

pub struct ChainStateAggregate {}

impl ChainStateAggregate {
    pub fn apply(event: ChainStateEvent) {}
}
