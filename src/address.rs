use tdn::types::primitive::Result;

use crate::bytes::Bytes;

pub struct Address([u8; 32]);

impl Address {
    pub fn from_str(_s: &str) -> Result<Self> {
        Ok(Address([0u8; 32]))
    }
}

impl Bytes for Address {
    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut b = [0u8; 32];
        b.clone_from_slice(bytes);
        Ok(Address(b))
    }
}
