use tdn::types::primitive::Result;

use crate::transaction::{
    transfer::{TransferOutput, TransferTransaction},
    Output, Transaction,
};

pub struct Wallet {
    transfers: Vec<(f64, TransferOutput)>,
}

impl Wallet {
    pub fn load() -> Result<Wallet> {
        Ok(Wallet {
            transfers: vec![(1.0, TransferOutput {}), (2.0, TransferOutput {})],
        })
    }

    pub fn build_tx(&mut self, to: [u8; 32], amount: f64) -> Result<TransferTransaction> {
        let mut selected = vec![];
        let mut selected_amount: f64 = 0.0;

        for i in self.transfers.iter() {
            selected_amount += i.0;
            selected.push(i.1.to_input());
            if selected_amount >= amount {
                break;
            }
        }

        let outputs = vec![TransferOutput {}, TransferOutput {}];

        Ok(Transaction::new(selected, outputs))
    }
}
