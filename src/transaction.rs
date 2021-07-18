pub struct Input {
    //
}

pub struct Output {
    //
}

pub struct Transaction {
    hash: [u8; 32],
    inputs: Vec<Input>,
    outputs: Vec<Output>,
}
