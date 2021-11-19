use tdn::types::primitive::PeerAddr;

use crate::miner::Round;

pub struct Proof {
    superviser: PeerAddr,
    timestamp: u64,
    sign: Vec<u8>,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Distance {}

impl Distance {
    pub fn calc(left: &PeerAddr, right: &PeerAddr, round: &Round) -> Distance {
        todo!()
    }
}
