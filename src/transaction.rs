use tdn::types::primitive::Result;

use crate::address::Address;
use crate::bytes::Bytes;

const VERSION: u32 = 0u32;

pub mod anonymos;
//mod exchange;
//pub mod transfer;

pub enum ScriptType {
    Transfer,
    Anonymous,
    Exchange,
}

impl Bytes for ScriptType {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            ScriptType::Transfer => vec![0u8],
            ScriptType::Anonymous => vec![1u8],
            ScriptType::Exchange => vec![2u8],
        }
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        match bytes[0] {
            0u8 => Ok(ScriptType::Transfer),
            1u8 => Ok(ScriptType::Anonymous),
            2u8 => Ok(ScriptType::Exchange),
            _ => Err(anyhow!("deserialize ScriptType failure.")),
        }
    }
}

pub struct Transaction {
    version: u32,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    locktime: u32,
}

pub struct Input {
    proof: Vec<u8>,
}

pub struct Output {
    pub script: ScriptType,
    pub amount: u128,
    pub params: Vec<u8>,
}

impl Output {
    pub fn prove(&self) -> Input {
        Input { proof: vec![] }
    }
}

impl Transaction {
    pub fn new(inputs: Vec<Input>, outputs: Vec<Output>) -> Self {
        Transaction {
            inputs,
            outputs,
            version: VERSION,
            locktime: 0u32,
        }
    }

    pub fn valid(&self) -> bool {
        true
    }
}
