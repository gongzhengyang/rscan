use std::collections::HashSet;
use std::net::IpAddr;
use std::path::Path;
use std::sync::Arc;

use itertools::Itertools;
use snafu::ResultExt;
use tokio::sync::{Mutex, OnceCell};

use crate::err::{CommonIoSnafu, Result};

static RECEIVE_PACKETS: OnceCell<Arc<Mutex<HashSet<IpAddr>>>> = OnceCell::const_new();

pub async fn receive_packets_handle() -> &'static Arc<Mutex<HashSet<IpAddr>>> {
    RECEIVE_PACKETS
        .get_or_init(|| async { Arc::new(Mutex::new(HashSet::new())) })
        .await
}

pub async fn is_addr_received(addr: &IpAddr) -> bool {
    receive_packets_handle().await.lock().await.contains(addr)
}

pub async fn add_receive_ipaddr(addr: IpAddr) -> bool {
    let mut receive_handle = receive_packets_handle().await.lock().await;
    receive_handle.insert(addr)
}

pub async fn save_receive_addrs(path: &str) -> Result<()> {
    let cache = receive_packets_handle().await;
    let addrs = cache
        .lock()
        .await
        .iter()
        .map(|x| x.to_string())
        .collect_vec();
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        if !path.exists() {
            tokio::fs::create_dir_all(parent)
                .await
                .context(CommonIoSnafu)?;
        }
    }
    let results = addrs.join("\n");
    tracing::debug!("receive data :{results}");
    tokio::fs::write(path, results)
        .await
        .context(CommonIoSnafu)?;
    Ok(())
}
