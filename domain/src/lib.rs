pub mod aggregates;
pub mod entities;
pub mod events;
pub mod value_objects;

#[cfg(test)]
mod aggregates_tests {
    use super::aggregates::*;
    use super::entities::*;
    use super::events::*;
    use super::value_objects::*;

    #[test]
    fn new() {
        let mut a = ChainStateAggregate::default();
        a.apply(&ChainStateEvent::UtxoConfirmed {
            utxo_id: todo!(),
            value: todo!(),
            confirmed_block_height: todo!(),
            confirmed_block_hash: todo!(),
        });
    }
}
