use tdn::{
    smol::channel::Sender,
    types::{
        message::SendType,
        primitive::{PeerAddr, Result},
    },
};

use crate::miner::Round;
use crate::proof::{Distance, Proof};

struct Miner {
    addr: PeerAddr,
    supervised: bool,
}

struct LightTable {
    //
}

struct Group {
    my: PeerAddr,
    miners: Vec<Miner>,
    lights: LightTable,
}

pub enum Event {
    /// receive other want to send amount to me.
    /// params: amount.
    Receive(u128),
    /// receive other's receipt about send amount.
    /// params: random public_key, amount.
    Receipt([u8; 32], u128),
    /// receive transation.
    Transaction,
    Block,
    AssertOnline,
    CheckOnline,
    ProveOnline,
    RequestProve,
}

impl Group {
    fn handle() {
        todo!()
    }
}

impl Group {
    /// update need supervise miners in next round.
    fn supervise_update(&mut self, prev_round: &Round) {
        // number = ceil(log(self.miners.len(), 2)) > 3
        let lognum = if self.miners.len() > 16 {
            (self.miners.len() as f32).log2().ceil() as usize
        } else {
            4
        };

        let mut distances: Vec<(Distance, usize)> = self
            .miners
            .iter()
            .enumerate()
            .map(|(index, miner)| (Distance::calc(&self.my, &miner.addr, prev_round), index))
            .collect();
        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        //distances.sort();
        for i in 0..lognum {
            self.miners[distances[i].1].supervised = true;
        }
    }

    /// build supervise message for need supervised miner.
    fn supervise(&self) -> Sender<SendType> {
        todo!()
    }

    /// handle supervise feedback.
    /// if ok, return the proof.
    /// if err. do nothing.
    fn supervise_feedback() -> Result<Proof> {
        todo!()
    }

    /// check miner's online proof is valid.
    fn valid_online_proofs(&self, proofs: Vec<Proof>, round: &Round) -> bool {
        todo!()
    }
}
