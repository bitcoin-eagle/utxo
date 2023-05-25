use crate::value_objects::*;

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
    pub fn new() -> Self {
        todo!();
        Self {
            id: bitcoin::OutPoint::null().into(),
            value: todo!(),
            spendable: todo!(),
            confirmed_block_height: todo!(),
            confirmed_block_hash: todo!(),
            spent_block_height: todo!(),
            spent_block_hash: todo!(),
        }
    }
}
