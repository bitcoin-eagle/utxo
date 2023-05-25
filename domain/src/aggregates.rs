use crate::{entities::*, events::*, value_objects::*};

use bitcoin::Network;
use std::collections::HashMap;

#[readonly::make]
pub struct ChainStateAggregate {
    pub network: Network,
    pub utxos: HashMap<UtxoId, UtxoEntity>,
}

impl ChainStateAggregate {
    pub fn apply(event: ChainStateEvent) {
        match event {
            ChainStateEvent::UtxoConfirmed {} => todo!(),
            ChainStateEvent::UtxoUnconfirmed {} => todo!(),
            ChainStateEvent::UtxoSpent {} => todo!(),
            ChainStateEvent::UtxoUnspent {} => todo!(),
        }
    }
}
