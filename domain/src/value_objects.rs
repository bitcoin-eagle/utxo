use bitcoin::{
    hashes::{sha256::HashEngine, sha256d, Hash},
    OutPoint,
};

#[readonly::make]
pub struct UtxoId(OutPoint);
#[readonly::make]
pub struct Txid(bitcoin::Txid);
#[readonly::make]
pub struct BlockHash(bitcoin::BlockHash);

impl UtxoId {
    pub fn new(value: OutPoint) -> Self {
        Self(value)
    }
}

impl From<OutPoint> for UtxoId {
    fn from(value: OutPoint) -> Self {
        Self(value)
    }
}

impl Default for UtxoId {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl From<bitcoin::Txid> for Txid {
    fn from(value: bitcoin::Txid) -> Self {
        Self(value)
    }
}

impl Default for Txid {
    fn default() -> Self {
        Self(bitcoin::Txid::from_raw_hash(sha256d::Hash::from_engine(
            HashEngine::default(),
        )))
    }
}

impl BlockHash {
    pub fn new(value: bitcoin::BlockHash) -> Self {
        Self(value)
    }
}