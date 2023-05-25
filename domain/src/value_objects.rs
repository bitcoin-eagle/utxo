use bitcoin::OutPoint;

#[readonly::make]
pub struct UtxoId(OutPoint);

impl From<OutPoint> for UtxoId {
    fn from(value: OutPoint) -> Self {
        Self(value)
    }
}

impl UtxoId {
    pub fn new(outpoint: OutPoint) -> Self {
        Self(outpoint)
    }
}
