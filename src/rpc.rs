use std::sync::Arc;
use tdn::{
    smol::lock::RwLock,
    types::{
        primitive::HandleResult,
        rpc::{json, RpcHandler, RpcParam},
    },
};

use crate::wallet::Wallet;

pub struct State {
    wallet: RwLock<Wallet>,
}

pub fn inject_rpc(wallet: Wallet) -> RpcHandler<State> {
    let mut rpc_handler = RpcHandler::new(State {
        wallet: RwLock::new(wallet),
    });

    rpc_handler.add_method("echo", |params, _state: Arc<State>| async move {
        Ok(HandleResult::rpc(json!(params)))
    });

    rpc_handler.add_method(
        "new-tx",
        |params: Vec<RpcParam>, state: Arc<State>| async move {
            let to = params[0].as_str()?; // to address
            let amount = params[1].as_f64()?; // amount

            let tx = state.wallet.write().await.build_tx([0u8; 32], amount)?;
            println!("{:?}", tx);

            Ok(HandleResult::rpc(json!({"to": to, "amount": amount})))
        },
    );

    rpc_handler
}
