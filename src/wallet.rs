use tdn::types::primitive::Result;

use crate::address::Address;
use crate::bytes::Bytes;
use crate::transaction::{anonymos::AnonymosProof, Output, ScriptType, Transaction};

pub struct Wallet {
    outputs: Vec<Output>,
}

impl Wallet {
    pub fn load() -> Result<Wallet> {
        Ok(Wallet { outputs: vec![] })
    }

    pub fn build_transfer_tx(&mut self, to: Address, amount: u128) -> Result<Transaction> {
        let mut selected = vec![];
        let mut selected_amount: u128 = 0;

        for i in self.outputs.iter() {
            selected_amount += i.amount;
            selected.push(i.prove());
            if selected_amount >= amount {
                break;
            }
        }

        let outputs = vec![
            Output {
                script: ScriptType::Transfer,
                amount: 10,
                params: to.to_bytes(),
            },
            Output {
                script: ScriptType::Transfer,
                amount: 10,
                params: vec![],
            },
        ];

        Ok(Transaction::new(selected, outputs))
    }

    pub fn _build_anonymos_tx(
        &mut self,
        _proof: AnonymosProof,
        _amount: u128,
    ) -> Result<Transaction> {
        todo!()
    }
}
