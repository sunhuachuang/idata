#[derive(Debug)]
pub struct Input {
    //
}

#[derive(Debug)]
pub struct Output {
    //
}

impl Output {
    pub fn to_input(&self) -> Input {
        Input {}
    }
}

#[derive(Debug)]
pub struct Transaction {
    hash: [u8; 32],
    inputs: Vec<Input>,
    outputs: Vec<Output>,
}

impl Transaction {
    pub fn new(inputs: Vec<Input>, outputs: Vec<Output>) -> Self {
        Self {
            hash: [0u8; 32],
            inputs,
            outputs,
        }
    }
}
