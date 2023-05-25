use crate::value_objects::*;

#[readonly::make]
pub struct UtxoEntity {
    pub id: UtxoId,
}

impl UtxoEntity {
    pub fn new() -> Self {
        todo!();
        Self {
            id: bitcoin::OutPoint::null().into(),
        }
    }
}
