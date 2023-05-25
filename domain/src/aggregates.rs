use crate::events::BestChainStateEvent;

pub struct BestChainStateAggregate {}

impl BestChainStateAggregate {
    pub fn apply(&mut self, event: BestChainStateEvent) {
        match event {
            BestChainStateEvent::UtxoConfirmed {} => todo!(),
            BestChainStateEvent::UtxoUnconfirmed {} => todo!(),
            BestChainStateEvent::UtxoSpent {} => todo!(),
            BestChainStateEvent::UtxoUnspent {} => todo!(),
        }
    }
}
