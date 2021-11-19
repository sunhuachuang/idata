use super::{Input, Output, Transaction};

/// token transfer transaction.
pub struct TransferTransaction {
    hash: [u8; 32],
    inputs: Vec<TransferInput>,
    outputs: Vec<TransferOutput>,
}

#[derive(Debug)]
pub struct TransferInput {
    //
}

impl Input for TransferInput {}

pub struct TransferOutput {
    //
}

impl Output for TransferOutput {
    type Input = TransferInput;

    fn to_input(&self) -> Self::Input {
        TransferInput {}
    }
}

impl Transaction for TransferTransaction {
    type Input = TransferInput;
    type Output = TransferOutput;

    fn new(inputs: Vec<Self::Input>, outputs: Vec<Self::Output>) -> Self {
        Self {
            hash: [0u8; 32],
            inputs,
            outputs,
        }
    }
}
