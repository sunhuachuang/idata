use std::sync::Arc;
use tdn::types::{
    primitive::HandleResult,
    rpc::{json, RpcHandler},
};

pub struct State(u32);

pub fn inject_rpc() -> RpcHandler<State> {
    let mut rpc_handler = RpcHandler::new(State(1));
    rpc_handler.add_method("echo", |params, state: Arc<State>| async move {
        assert_eq!(1, state.0);
        Ok(HandleResult::rpc(json!(params)))
    });

    rpc_handler.add_method("say_hello", |_params, state: Arc<State>| async move {
        assert_eq!(1, state.0);
        Ok(HandleResult::rpc(json!("hello")))
    });

    rpc_handler
}
