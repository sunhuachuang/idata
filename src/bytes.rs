use tdn::types::primitive::Result;

pub trait Bytes: Sized {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Result<Self>;
}
