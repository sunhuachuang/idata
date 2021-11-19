//mod anonymos;
//mod exchange;
pub mod transfer;

pub trait Input {}

pub trait Output {
    type Input: Input;
    fn to_input(&self) -> Self::Input;
}

pub trait Transaction {
    type Input: Input;
    type Output: Output;

    fn new(inputs: Vec<Self::Input>, outputs: Vec<Self::Output>) -> Self;
}
