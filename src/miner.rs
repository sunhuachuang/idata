mod primitive;

use std::path::PathBuf;
use tdn::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let db_path = PathBuf::from("./.miner");
    if !db_path.exists() {
        tokio::fs::create_dir_all(&db_path).await?;
    }
    let mut config = Config::load_save(db_path.clone()).await;
    config.db_path = Some(db_path);
    //config.p2p_allowlist.append(&mut network_seeds());
    config.group_ids = vec![primitive::MINER_ID];

    let (peer_addr, _send, mut out_recv) = start_with_config(config).await?;
    println!("Example: peer id: {}", peer_addr.short_show());

    while let Some(message) = out_recv.recv().await {
        match message {
            ReceiveMessage::Group(msg) => match msg {
                RecvType::Connect(peer, _data) => {
                    println!("receive group peer {} join", peer.id.short_show());
                }
                RecvType::Result(..) => {
                    //
                }
                RecvType::Leave(peer) => {
                    println!("receive group peer {} leave", peer.short_show());
                }
                RecvType::Event(peer, _data) => {
                    // if let Ok(handle_result) = group.write().await.handle(peer, data) {
                    //     handle(handle_result, now_rpc_uid, true, &sender).await;
                    // }
                    println!("receive group event from {}", peer.short_show());
                }
                _ => {}
            },
            ReceiveMessage::Layer(gid, msg) => match msg {
                RecvType::Connect(peer, _data) => {
                    println!(
                        "Layer Join: {}, Addr: {}.",
                        gid.short_show(),
                        peer.id.short_show()
                    );
                }
                RecvType::Result(..) => {
                    //
                }
                _ => {}
            },
            ReceiveMessage::NetworkLost => {
                // println!("No network connections");
            }
            ReceiveMessage::Rpc(..) => {
                //
            }
        }
    }

    Ok(())
}
