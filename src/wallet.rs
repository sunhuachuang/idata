use std::io::Result;

use crate::transaction::{Output, Transaction};

pub struct Wallet {
    outputs: Vec<(f64, Output)>,
}

impl Wallet {
    pub fn load() -> Result<Wallet> {
        Ok(Wallet {
            outputs: vec![(1.0, Output {}), (2.0, Output {})],
        })
    }

    pub fn build_tx(&mut self, to: [u8; 32], amount: f64) -> Result<Transaction> {
        let mut selected = vec![];
        let mut selected_amount: f64 = 0.0;

        for i in self.outputs.iter() {
            selected_amount += i.0;
            selected.push(i.1.to_input());
            if selected_amount >= amount {
                break;
            }
        }

        let outputs = vec![Output {}, Output {}];

        Ok(Transaction::new(selected, outputs))
    }
}
