use std::net::IpAddr;
use std::sync::{Arc, Mutex};

use hashbrown::HashSet;
use tokio::sync::OnceCell;

static RECEIVE_PACKETS: OnceCell<Arc<Mutex<HashSet<IpAddr>>>> = OnceCell::const_new();

pub async fn receive_packets_handle() -> &'static Arc<Mutex<HashSet<IpAddr>>> {
    RECEIVE_PACKETS
        .get_or_init(|| async { Arc::new(Mutex::new(HashSet::new())) })
        .await
}

pub async fn is_addr_received(addr: &IpAddr) -> bool {
    receive_packets_handle()
        .await
        .lock()
        .unwrap()
        .contains(addr)
}

pub async fn add_receive_ipaddr(addr: IpAddr) {
    let mut receive_handle = receive_packets_handle().await.lock().unwrap();
    receive_handle.insert(addr);
}
