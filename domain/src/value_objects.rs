use bitcoin::{
    hashes::{sha256::HashEngine, sha256d, Hash},
    OutPoint,
};

#[readonly::make]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct UtxoId(pub OutPoint);

#[readonly::make]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Txid(pub bitcoin::Txid);

#[readonly::make]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockHash(pub bitcoin::BlockHash);

pub type Network = bitcoin::Network;

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

impl Default for BlockHash {
    fn default() -> Self {
        Self(bitcoin::BlockHash::all_zeros())
    }
}
