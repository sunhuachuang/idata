mod generator;
mod group;
mod miner;
mod proof;
mod rpc;
mod transaction;
mod wallet;

use tdn::prelude::*;

fn main() {
    tdn::smol::block_on(async {
        let (peer_addr, send, out_recv) = start().await.unwrap();
        println!("Example: peer id: {}", peer_addr.short_show());

        let rpc_handler = rpc::inject_rpc(wallet::Wallet::load().unwrap());
        // tdn::smol::spawn(miner::start_miner()).detach();

        while let Ok(message) = out_recv.recv().await {
            match message {
                ReceiveMessage::Group(msg) => match msg {
                    RecvType::Connect(peer, _data) => {
                        println!("receive group peer {} join", peer.short_show());
                    }
                    RecvType::Result(..) => {
                        //
                    }
                    RecvType::Leave(peer) => {
                        println!("receive group peer {} leave", peer.short_show());
                    }
                    RecvType::Event(peer, data) => {
                        if let Ok(handle_result) = group.write().await.handle(peer, data) {
                            handle(handle_result, now_rpc_uid, true, &sender).await;
                        }
                        println!("receive group event from {}", peer.short_show());
                    }
                    _ => {}
                },
                ReceiveMessage::Layer(gid, msg) => match msg {
                    RecvType::Connect(peer, _data) => {
                        println!(
                            "Layer Join: {}, Addr: {}.",
                            gid.short_show(),
                            peer.short_show()
                        );
                    }
                    RecvType::Result(..) => {
                        //
                    }
                    _ => {}
                },
                ReceiveMessage::Rpc(uid, params, is_ws) => {
                    if let Ok(HandleResult {
                        mut rpcs,
                        groups: _,
                        layers: _,
                        networks: _,
                    }) = rpc_handler.handle(params).await
                    {
                        loop {
                            if rpcs.len() != 0 {
                                let msg = rpcs.remove(0);
                                send.send(SendMessage::Rpc(uid, msg, is_ws))
                                    .await
                                    .expect("TDN channel closed");
                            } else {
                                break;
                            }
                        }
                    }
                }
                ReceiveMessage::NetworkLost => {
                    // println!("No network connections");
                }
            }
        }
    });
}
